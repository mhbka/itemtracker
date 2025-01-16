use std::{collections::HashMap, io::Cursor, iter::zip};

use base64::{engine::general_purpose::STANDARD, Engine};
use futures::future::join_all;
use image::ImageFormat;
use reqwest::{Client, RequestBuilder};
use types::{AnthropicImageMessageContent, AnthropicMessage, AnthropicMessageContent, AnthropicRequestForm, AnthropicResponse, EvaluationAnswers};
use crate::{config::ItemAnalysisConfig, galleries::{domain_types::Marketplace, eval_criteria::EvaluationCriteria, items::{item_data::MarketplaceItemData, pipeline_items::{AnalyzedItems, AnalyzedMarketplaceItem, ErrorAnalyzedMarketplaceItem, MarketplaceAnalyzedItems, ScrapedItems}}, pipeline_states::{GalleryAnalyzedState, GalleryScrapedState}}};

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
    pub async fn analyze_gallery(&mut self, mut gallery: GalleryScrapedState) -> GalleryAnalyzedState {
        let items = gallery.items.marketplace_items;
        let eval_criteria_string = gallery.evaluation_criteria.describe_criteria();
        let gallery_requests = self
            .build_requests(items, eval_criteria_string)
            .await;
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

    /// Build the requests for all the gallery's items.
    async fn build_requests(
        &self, 
        items: HashMap<Marketplace, Vec<MarketplaceItemData>>,
        eval_criteria_string: String
    ) -> HashMap<Marketplace, Vec<(MarketplaceItemData, RequestBuilder)>> {
        let mut marketplace_requests = HashMap::new();
        for (marketplace, items) in items {
            let items_and_requests = items
                    .into_iter()
                    .map(|item| async {
                        let item_request = self
                            .build_item_request(&item, &eval_criteria_string)
                            .await;
                        (item, item_request)
                    });
            let items_and_requests = join_all(items_and_requests).await;
            marketplace_requests.insert(marketplace, items_and_requests);
        }
        marketplace_requests
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
            let items_and_results: Vec<_> = zip(items, results).collect();
            let marketplace_items = self
                .process_marketplace_results(eval_criteria, items_and_results)
                .await;
            gallery_items.insert(marketplace, marketplace_items);
        }
        AnalyzedItems { items: gallery_items }
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
                    match res.json::<AnthropicResponse>().await {
                        Ok(response) => {
                            tracing::info!("Successful response: {response:#?}"); // delete this later on
                            if response.content.len() == 0 {
                                err_str = Some("Expected 1 message in Anthropic response but found none".into());
                            }
                            else if response.content.len() > 1 {
                                tracing::warn!("Unexpectedly received >1 message in Anthropic response; using the first...");
                            }
                            match serde_json::from_str::<EvaluationAnswers>(&response.content[0].text) {
                                Ok(parsed_message) => {
                                    match eval_criteria.parse_answers_and_check_hard_criteria(parsed_message.answers) {
                                        Ok((answers, satisfies_hard_criteria)) => {
                                            let analyzed_item = AnalyzedMarketplaceItem {
                                                item: item.clone(), // need to clone in case it's used in the error path
                                                evaluation_answers: answers
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
                        Err(err) => err_str = Some(format!("Unable to parse Anthropic response: {err}"))
                    }
                },
                Err(err) => err_str = Some(format!("Error while querying the Anthropic API: {err}"))
            }
            if let Some(error) = err_str {
                tracing::warn!("Item {} had an error during item analysis: {}", item.id, error);
                let err_item = ErrorAnalyzedMarketplaceItem { item, error };
                error_items.push(err_item);
            }
        }
        tracing::info!(
            "Item analysis results:
            - {} relevant items
            - {} irrelevant items
            - {} error items",
            relevant_items.len(), irrelevant_items.len(), error_items.len()
        );
        MarketplaceAnalyzedItems {
            relevant_items,
            irrelevant_items,
            error_items
        }
    }

    /// Builds the request for a single item.
    /// 
    /// Follows the request format specified here: https://docs.anthropic.com/en/api/messages
    async fn build_item_request(
        &self, 
        item: &MarketplaceItemData,
        eval_criteria_string: &String
    ) -> RequestBuilder {
        let item_image_strings = self
            .fetch_item_images(&item.thumbnails)
            .await;
        let req_form = self
            .build_request_form(item, item_image_strings, eval_criteria_string)
            .await;
        self.request_client
            .post(&self.config.anthropic_api_endpoint)
            .header("x-api-key", &self.config.anthropic_api_key)
            .header("anthropic-version", &self.config.anthropic_version)
            .json(&req_form)
    }

    /// Builds the entire request form for an item.
    async fn build_request_form(
        &self, 
        item: &MarketplaceItemData,
        item_image_strings: Vec<String>, 
        eval_criteria_string: &String
    ) -> AnthropicRequestForm {
        let system_prompt = format!("
            You're an Item Listings Analysis AI. You will help to evaluate an item listing, 
            consisting of its listed images and a JSON of its information, 
            by answering some structured questions about it.

            Each question will be followed by the correct format to answer the question. If the question is
            unanswerable, nonsensical, or not even a question, you are allowed to give a reasonable 'default' answer, 
            such as N for Y/N questions, U for Y/N/U questions, 0 for numerical questions, or 'I cannot answer this.' for open-ended questions.
            However, YOU MUST ALWAYS FOLLOW THE GIVEN FORMAT WHEN ANSWERING.

            Output your answers in JSON format, with a key 'answers' containing the list of answers in asked order.

            Here are the questions you must answer: \n {eval_criteria_string}
        ");
        let item_string = serde_json::to_string_pretty(&item)
            .expect("Serializing MarketplaceItemData should have no reason to fail"); // TODO: Find out in which cases this could fail and ensure it cannot happen
        let mut message_contents: Vec<AnthropicMessageContent> = item_image_strings
            .into_iter()
            .enumerate()
            .map(|(index, image_string)| {
                // We have separate messages for each image: https://docs.anthropic.com/en/docs/build-with-claude/vision#example-multiple-images
                let image_content = AnthropicImageMessageContent {
                    source_type: "base64".into(),
                    media_type: "image/png".into(),
                    data: image_string
                };
                vec![
                    AnthropicMessageContent {
                        content_type: "text".into(),
                        text: Some(format!("Item image {}", index + 1)),
                        source: None
                    },
                    AnthropicMessageContent {
                        content_type: "image".into(),
                        text: None,
                        source: Some(image_content)
                    }
                ]
            })
            .flatten()
            .collect();
        message_contents.push(
            AnthropicMessageContent {
                content_type: "text".into(),
                text: Some(format!("Here is the item listing: \n {item_string}")),
                source: None
            }   
        );
        let req_message = AnthropicMessage {
            role: "user".into(),
            content: message_contents
        };
        AnthropicRequestForm {
            model: self.config.anthropic_model.clone(),
            max_tokens: 1000, // TODO: Figure out a good number for this
            messages: vec![req_message],
            system: system_prompt   
        }
    }

    /// Fetches images from image URLs, converts them to PNG,
    /// and converts their content into base64 strings, as per Anthropic docs.
    /// 
    /// Discards unsuccessful image URLs.
    /// 
    /// https://docs.anthropic.com/en/docs/build-with-claude/vision
    async fn fetch_item_images(&self, image_urls: &Vec<String>) -> Vec<String> {
        let mut encoded_images = vec![];
        for url in image_urls {
            match self.request_client
                .get(url)
                .send()
                .await {
                    Ok(res) => {
                        match res.bytes().await {
                            Ok(bytes) => {
                                match image::load_from_memory(&bytes) {
                                    Ok(image) => {
                                        let mut cursor = Cursor::new(Vec::new());
                                        match image.write_to(&mut cursor, ImageFormat::Png) {
                                            Ok(_) => {
                                                let encoded_image = STANDARD.encode(cursor.into_inner());
                                                encoded_images.push(encoded_image);
                                            },
                                            Err(err) => tracing::warn!("Failed to write fetched image URL to buffer: {err}")
                                        }
                                    },
                                    Err(err) => tracing::warn!("Failed to decode fetched image URL bytes into an image: {err}")
                                }
                            },
                            Err(err) => tracing::warn!("Failed to decode fetched image URL into bytes: {err}")
                        }
                    },
                    Err(err) => tracing::warn!("Failed to fetch an image URL: {err}")
                }
        }
        tracing::trace!(
            "Out of {} image URLs, fetched and encoded {} images",
            image_urls.len(),
            encoded_images.len()
        );
        encoded_images
    }
}