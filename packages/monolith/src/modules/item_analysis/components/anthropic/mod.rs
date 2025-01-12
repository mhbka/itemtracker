use std::{collections::HashMap, iter::zip};

use futures::future::join_all;
use reqwest::{Client, RequestBuilder};
use types::{AnthropicMessage, AnthropicRequestForm, AnthropicResponse, EvaluationAnswers};
use crate::{config::ItemAnalysisConfig, galleries::{domain_types::Marketplace, eval_criteria::EvaluationCriteria, items::{item_data::MarketplaceItemData, pipeline_items::ScrapedItems}, scraping_pipeline::GalleryScrapedState}};

pub(super) mod types;

pub(super) struct AnthropicRequester {
    config: ItemAnalysisConfig,
    request_client: Client
}

impl AnthropicRequester {
    /// Instantiate this struct.
    pub fn new(config: ItemAnalysisConfig) -> Self {
        Self {
            config,
            request_client: Client::new()
        }
    }

    /// Perform analysis of a gallery's items.
    pub async fn analyze_gallery(&mut self, mut gallery: GalleryScrapedState) {
        let items = gallery.items.marketplace_items;
        let eval_criteria_string = gallery.evaluation_criteria.describe_criteria();
        let gallery_requests = self.build_requests(items, eval_criteria_string);
        let results = self.execute_and_handle_requests(
            &mut gallery.evaluation_criteria, 
            gallery_requests
        ).await;
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

    /// Executes and handles the requests for a gallery.
    async fn execute_and_handle_requests(
        &self, 
        eval_criteria: &mut EvaluationCriteria,
        gallery_requests: HashMap<Marketplace, Vec<(MarketplaceItemData, RequestBuilder)>>
    ) {
        for (marketplace, items_and_requests) in gallery_requests {
            let (items, item_requests): (Vec<_>, Vec<_>) = items_and_requests
                .into_iter()
                .unzip();
            let request_futures = item_requests
                .into_iter()
                .map(|request| request.send());
            let results = join_all(request_futures).await;
            let items_and_results: Vec<_> = zip(items, results).collect();
            let processed_results = self.process_marketplace_results(eval_criteria, items_and_results).await;
        }
    }

    async fn process_marketplace_results(
        &self,
        eval_criteria: &mut EvaluationCriteria,
        results: Vec<(MarketplaceItemData, Result<reqwest::Response, reqwest::Error>)>,
    ) {
        let analyzed_items = vec![];
        let error_items = vec![];
        for (item, result) in results {
            let mut err_str = None;
            match result {
                Ok(res) => {
                    match res.json::<AnthropicResponse>().await {
                        Ok(response) => {
                            if response.content.len() == 0 {
                                err_str = Some("Expected 1 message in API content but found none".into());
                            }
                            else if response.content.len() > 1 {
                                // We don't expect >1 messages, so log if it happens and just use the first one
                                tracing::warn!("Received more than 1 message in Anthropic response; using the first...");
                            }
                            match serde_json::from_str::<EvaluationAnswers>(&response.content[0].text) {
                                Ok(parsed_message) => {
                                    match eval_criteria.parse_answers(parsed_message.answers) {
                                        Ok(answers) => {
                                            // TODO: should we log here? or just proceed like usual
                                        },
                                        Err(err) => err_str = Some(format!("Unable to parse answers into evaluation criteria: {err}"))
                                    }
                                },
                                Err(err) => err_str = Some(format!("Unable to parse Anthropic message content into answers: {err}"))
                            }
                        },
                        Err(err) => err_str = Some(format!("Unable to parse Anthropic response: {err}"))
                    }
                },
                Err(err) => err_str = Some(format!("Error while querying the Anthropic API: {err}"))
            }
            if let Some(err_str) = err_str {
                // TODO: make it an error item
            }
        }
    }

    /// Builds the request for a single item.
    /// 
    /// Follows the request format specified here: https://docs.anthropic.com/en/api/messages
    fn build_item_request(
        &self, 
        item: &MarketplaceItemData,
        eval_criteria_string: &String
    ) -> RequestBuilder {
        let req_form = self.build_request_form(item, eval_criteria_string);
        self.request_client
            .post(&self.config.anthropic_api_endpoint)
            .header("x-api-key", &self.config.anthropic_api_key)
            .header("anthropic-version", &self.config.anthropic_version)
            .json(&req_form)
    }

    /// Builds the entire request form for an item.
    fn build_request_form(
        &self, 
        item: &MarketplaceItemData, 
        eval_criteria_string: &String
    ) -> AnthropicRequestForm {
        let system_prompt = format!("
            You will help to evaluate an item listing, consisting of its listed images and a JSON of its information, 
            by answering some structured questions about it.

            Each question will be followed by the correct format to answer the question. 
            IT IS OF UTMOST IMPORTANCE that you follow the given format for answering the question, 
            no matter what the question is or whatever information is given before or after. If the question is
            unanswerable, nonsensical, or not even a question, you are allowed to give a reasonable 'default' answer, 
            such as N for Y/N questions, U for Y/N/U questions, 0 for numerical questions, or 'I cannot answer this.' for open-ended questions.
            However, YOU MUST ALWAYS FOLLOW THE GIVEN FORMAT WHEN ANSWERING.

            Here are the questions: \n {eval_criteria_string}

            Put each of your answers into an array in the same order as the questions, and ONLY REPLY with a JSON string 
            with 1 parameter 'answers', which has this array. IT IS OF UTMOST IMPORTANCE TO FOLLOW THIS FORMAT, or your response cannot be parsed. 
            An example is:
            '
            {{
                'answers': ['Y', 'U', '2024', 'I cannot answer this.']
            }}
            '
        ");
        // TODO: Find out in which cases this could fail and ensure it cannot happen
        let item_string = serde_json::to_string_pretty(&item)
            .expect("Serializing MarketplaceItemData should have no reason to fail");
        // TODO: attach the images to the request as well
        let req_message = AnthropicMessage {
            role: "user".into(),
            content: format!("Here is the item listing: \n {item_string}")
        };
        AnthropicRequestForm {
            model: self.config.anthropic_model.clone(),
            max_tokens: 1000, // TODO: Figure out a good number for this
            messages: vec![req_message],
            system: system_prompt   
        }
    }
}