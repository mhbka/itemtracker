use std::{collections::HashMap, iter::zip};

use futures::future::join_all;
use reqwest::{Client, RequestBuilder, StatusCode};
use types::{OpenAIImageURLMessage, OpenAIMessage, OpenAIMessageContent, OpenAIRequestForm, OpenAIResponse};
use crate::{config::ItemAnalysisConfig, domain::{domain_types::Marketplace, eval_criteria::EvaluationCriteria, item_data::MarketplaceItemData, pipeline_items::{AnalyzedMarketplaceItem, ErrorAnalyzedMarketplaceItem, MarketplaceAnalyzedItems}, pipeline_states::{GalleryItemEmbedderState, GalleryItemAnalysisState}}, scraping_pipeline::item_analysis::analyzer::anthropic::types::EvaluationAnswers};

mod types;

pub(super) struct OpenAIRequester {
    config: ItemAnalysisConfig,
    request_client: Client
}

impl OpenAIRequester {
    /// Instantiate the requester.
    pub fn new(config: ItemAnalysisConfig) -> Self {
        Self {
            config: config,
            request_client: Client::new()
        }
    }

    /// Perform analysis of a gallery's items.
    pub async fn analyze_gallery(&mut self, mut gallery: GalleryItemAnalysisState) -> GalleryItemEmbedderState {
        let items = gallery.items;
        let eval_criteria_string = gallery.evaluation_criteria.describe_criteria();
        let gallery_requests = self.build_requests(items, eval_criteria_string);
        let analyzed_items = self.execute_and_handle_requests(
            &mut gallery.evaluation_criteria, 
            gallery_requests
        ).await;
        GalleryItemEmbedderState {
            gallery_id: gallery.gallery_id,
            items: analyzed_items,
            failed_marketplace_reasons: gallery.failed_marketplace_reasons,
            marketplace_updated_datetimes: gallery.marketplace_updated_datetimes,
            used_evaluation_criteria: gallery.evaluation_criteria
        }
    }

    /// Executes and handles the requests for a gallery.
    async fn execute_and_handle_requests(
        &self, 
        eval_criteria: &mut EvaluationCriteria,
        gallery_requests: HashMap<Marketplace, Vec<(MarketplaceItemData, RequestBuilder)>>
    ) -> HashMap<Marketplace, MarketplaceAnalyzedItems> {
        let mut gallery_items = HashMap::new();
        for (marketplace, items_and_requests) in gallery_requests {
            let (items, item_requests): (Vec<_>, Vec<_>) = items_and_requests
                .into_iter()
                .unzip();
            let request_futures = item_requests
                .into_iter()
                .map(|request| request.send());
            let results = join_all(request_futures).await;
            let items_and_results = zip(items, results).collect();
            let marketplace_items = self
                .process_marketplace_results(eval_criteria, items_and_results)
                .await;
            gallery_items.insert(marketplace, marketplace_items);
        }
        gallery_items
    }

    /// Process the raw LLM output for all items in a gallery's marketplace.
    async fn process_marketplace_results(
        &self,
        eval_criteria: &mut EvaluationCriteria,
        results: Vec<(MarketplaceItemData, Result<reqwest::Response, reqwest::Error>)>,
    ) -> MarketplaceAnalyzedItems {
        let mut relevant_items = vec![];
        let mut irrelevant_items = vec![];
        let mut error_items = vec![];
        for (item, result) in results {
            let mut err_str = None;
            match result {
                Ok(res) => {
                    match res.status() {
                        StatusCode::OK => {
                            match res.json::<OpenAIResponse>().await {
                                Ok(response) => {
                                    tracing::trace!("Successful response: {response:#?}"); // TODO: delete this later on
                                    if response.choices.len() == 0 {
                                        err_str = Some("Expected 1 message in Anthropic response but found none".into());
                                    }
                                    else if response.choices.len() > 1 {
                                        tracing::warn!("Unexpectedly received >1 messages in Anthropic response; using the first...");
                                    }
                                    match &response.choices[0].text {
                                        Some(text) => {
                                            match serde_json::from_str::<EvaluationAnswers>(text) {
                                                Ok(parsed_message) => {
                                                    match eval_criteria.parse_answers_and_check_hard_criteria(parsed_message.answers) {
                                                        Ok((answers, satisfies_hard_criteria)) => {
                                                            let analyzed_item = AnalyzedMarketplaceItem {
                                                                item: item.clone(), 
                                                                evaluation_answers: answers,
                                                                item_description: parsed_message.item_description,
                                                                best_fit_image: parsed_message.best_fit_image
                                                            };
                                                            if satisfies_hard_criteria {
                                                                relevant_items.push(analyzed_item);
                                                            } else {
                                                                irrelevant_items.push(analyzed_item);
                                                            }
                                                        },
                                                        Err(err) => err_str = Some(format!("Unable to parse answers into evaluation criteria: {err}"))
                                                    }
                                                },
                                                Err(err) => err_str = Some(format!("Unable to parse Anthropic message content into answers: {err}"))
                                            }
                                        },
                                        None => err_str = Some("Anthropic message content contained no `text` key".into())
                                    }
                                },
                                Err(err) => err_str = Some(format!("Unable to parse Anthropic response: {err:#?}"))
                            }
                        },
                        other => {
                            let res = res.text().await;
                            err_str = Some(format!("Received unexpected status code {other} from Anthropic API; response: {res:#?}"));
                        }
                    }
                },
                Err(err) => err_str = Some(format!("Error while querying the Anthropic API: {err}"))
            }
            if let Some(error) = err_str {
                tracing::trace!("Item {} had an error during item analysis: {}", item.item_id, error);
                let err_item = ErrorAnalyzedMarketplaceItem { item, error };
                error_items.push(err_item);
            }
        }
        tracing::debug!(
            "Item analysis results: {} relevant items, {} irrelevant items, and {} error items",
            relevant_items.len(), 
            irrelevant_items.len(), 
            error_items.len()
        );
        MarketplaceAnalyzedItems {
            relevant_items,
            irrelevant_items,
            error_items
        }
    }

    /// Build the requests for all the gallery's items.
    fn build_requests(
        &self, 
        items: HashMap<Marketplace, Vec<MarketplaceItemData>>,
        eval_criteria_string: String
    ) -> HashMap<Marketplace, Vec<(MarketplaceItemData, RequestBuilder)>> {
        items
            .into_iter()
            .map(|(marketplace, items)| {
                let items_and_requests = items
                    .into_iter()
                    .map(|item| {
                        let item_request = self.build_item_request(&item, &eval_criteria_string);
                        (item, item_request)
                    })
                    .collect();
                (marketplace, items_and_requests)
            })
            .collect()
    }

    /// Builds the request for a single item.
    /// 
    /// Follows the request format specified here: https://platform.openai.com/docs/api-reference/chat
    fn build_item_request(
        &self, 
        item: &MarketplaceItemData,
        eval_criteria_string: &String
    ) -> RequestBuilder {
        let req_form = self.build_request_form(item, eval_criteria_string);
        self.request_client
            .post(&self.config.openai_api_endpoint)
            .bearer_auth(&self.config.openai_api_key)
            .json(&req_form)
    }

    /// Builds the entire request form for an item.
    fn build_request_form(
        &self, 
        item: &MarketplaceItemData,
        eval_criteria_string: &String
    ) -> OpenAIRequestForm {
        let system_prompt = format!("
            You're an Item Listings Analysis AI. 
            
            You will help to evaluate an item listing, consisting of its listed images and a JSON of its information, by answering some structured questions about it. 
            
            There are the following question types:
            - YesNo: Answer with 'Y'/'N' only
            - YesNoUncertain: Answer with 'Y'/'N'/'U' only
            - Int: Answer with an integer within a string
            - Float: Answer with a float within a string
            - OpenEnded: Answer as best as you can, under 200 characters

            If the question is unanswerable, nonsensical, or not even a question, you are allowed to give a reasonable 'default' answer, 
            such as N for Y/N questions, U for Y/N/U questions, 0 for numerical questions, or 'I cannot answer this.' for open-ended questions.
            However, YOU MUST ALWAYS FOLLOW THE GIVEN FORMAT WHEN ANSWERING.

            Output your answers in JSON format, with a key 'answers' containing the list of answers in asked order.
            If there are no questions, return the list empty.

            Here are the questions you must answer:\n {eval_criteria_string}
        ");
        let system_message = OpenAIMessage {
            role: "developer".to_string(),
            content: vec![
                OpenAIMessageContent {
                    content_type: "text".to_string(),
                    text: Some(system_prompt),
                    image_url: None
                }
            ]
        };
        let mut message_contents: Vec<OpenAIMessageContent> = item.thumbnails
            .clone()
            .into_iter()
            .enumerate()
            .map(|(index, url)| {
                // Follows the recommended format for sending multiple images: https://docs.anthropic.com/en/docs/build-with-claude/vision#example-multiple-images
                vec![
                    OpenAIMessageContent {
                        content_type: "text".into(),
                        text: Some(format!("Item image {}: ", index + 1)),
                        image_url: None
                    },
                    OpenAIMessageContent {
                        content_type: "image".into(),
                        text: None,
                        image_url: Some(OpenAIImageURLMessage { url })
                    }
                ]
            })
            .flatten()
            .collect();
        let item_string = serde_json::to_string_pretty(&item)
            .expect("Serializing MarketplaceItemData should have no reason to fail"); // TODO: not 100% sure here; find out in which cases this could fail
        message_contents.push(
            OpenAIMessageContent {
                content_type: "text".into(),
                text: Some(format!("Here is the item listing: \n {item_string}")),
                image_url: None
            }   
        );
        let user_messages = OpenAIMessage {
            role: "user".into(),
            content: message_contents
        };
        OpenAIRequestForm {
            model: self.config.anthropic_model.clone(),
            max_completion_tokens: 1000, // TODO: Figure out a good number for this
            messages: vec![system_message, user_messages]
        }
    }
}