use std::{collections::HashMap, iter::zip};

use futures::future::join_all;
use reqwest::{Client, RequestBuilder};
use types::{OpenAIImageURLMessage, OpenAIMessage, OpenAIMessageContent, OpenAIRequestForm};

use crate::{config::ItemAnalysisConfig, galleries::{domain_types::Marketplace, eval_criteria::EvaluationCriteria, items::{item_data::MarketplaceItemData, pipeline_items::AnalyzedItems}, pipeline_states::{GalleryAnalyzedState, GalleryScrapedState}}};

mod types;

pub(super) struct OpenAIRequester {
    config: ItemAnalysisConfig,
    request_client: Client
}

impl OpenAIRequester {
    /// Instantiate the requester.
    pub fn new(config: &ItemAnalysisConfig) -> Self {
        Self {
            config: config.clone(),
            request_client: Client::new()
        }
    }

    /// Perform analysis of a gallery's items.
    pub async fn analyze_gallery(&mut self, mut gallery: GalleryScrapedState) -> GalleryAnalyzedState {
        let items = gallery.items.marketplace_items;
        let eval_criteria_string = gallery.evaluation_criteria.describe_criteria();
        let gallery_requests = self.build_requests(items, eval_criteria_string);
        let analyzed_items = self.execute_and_handle_requests(
            &mut gallery.evaluation_criteria, 
            gallery_requests
        ).await;
        GalleryAnalyzedState {
            gallery_id: gallery.gallery_id,
            items: analyzed_items,
            evaluation_criteria: gallery.evaluation_criteria
        }
    }

    /// Executes and handles the requests for a gallery.
    async fn execute_and_handle_requests(
        &self, 
        eval_criteria: &mut EvaluationCriteria,
        gallery_requests: HashMap<Marketplace, Vec<(MarketplaceItemData, RequestBuilder)>>
    ) -> AnalyzedItems {
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
        AnalyzedItems { items: gallery_items }
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

            Here are the questions you must answer: \n {eval_criteria_string}
        ");
        let system_message = OpenAIMessage {
            role: "developer".into(),
            content: vec![
                OpenAIMessageContent {
                    content_type: "text".into(),
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
            .expect("Serializing MarketplaceItemData should have no reason to fail"); // TODO: Find out in which cases this could fail and ensure it cannot happen
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