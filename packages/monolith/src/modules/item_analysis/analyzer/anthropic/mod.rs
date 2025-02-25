use std::{collections::HashMap, io::Cursor, iter::zip};

use base64::{engine::general_purpose::STANDARD, Engine};
use futures::future::join_all;
use image::ImageFormat;
use reqwest::{Client, RequestBuilder, StatusCode};
use types::{AnthropicImageMessageContent, AnthropicMessage, AnthropicMessageContent, AnthropicRequestForm, AnthropicResponse, EvaluationAnswers};
use crate::{config::ItemAnalysisConfig, galleries::{domain_types::Marketplace, eval_criteria::EvaluationCriteria, items::{item_data::MarketplaceItemData, pipeline_items::{AnalyzedMarketplaceItem, ErrorAnalyzedMarketplaceItem, MarketplaceAnalyzedItems}}}};

pub(super) mod types;

pub(super) struct AnthropicRequester {
    config: ItemAnalysisConfig,
    request_client: Client
}

impl AnthropicRequester {
    /// Instantiate the requester.
    pub fn new(config: ItemAnalysisConfig) -> Self {
        Self {
            config,
            request_client: Client::new()
        }
    }

    /// Perform analysis of a gallery's items.
    pub async fn analyze_gallery(
        &mut self,
        items: HashMap<Marketplace, Vec<MarketplaceItemData>>,
        eval_criteria: &EvaluationCriteria
    ) -> HashMap<Marketplace, MarketplaceAnalyzedItems> {
        let eval_criteria_string = eval_criteria.describe_criteria();
        let (gallery_requests, failed_image_items) = self
            .build_requests(items, eval_criteria_string)
            .await;
        self.execute_and_handle_requests(
            eval_criteria, 
            gallery_requests,
            failed_image_items
        ).await
    }

    /// Build the requests for all the gallery's items.
    /// 
    /// Returns the requests, as well as items whose images could not be fetched (as `ErrorAnalyzedMarketplaceItem`).
    async fn build_requests(
        &self, 
        items: HashMap<Marketplace, Vec<MarketplaceItemData>>,
        eval_criteria_string: String
    ) -> (
        HashMap<Marketplace, Vec<(MarketplaceItemData, RequestBuilder)>>,
        HashMap<Marketplace, Vec<ErrorAnalyzedMarketplaceItem>>
    ) {
        let mut marketplace_requests = HashMap::new();
        let mut marketplace_failed_image_items = HashMap::new();
        for (marketplace, items) in items {
            let mut failed_image_items = Vec::new();
            let item_requests = items
                    .into_iter()
                    .map(|item| async {
                        let item_request = self
                            .build_item_request(&item, &eval_criteria_string)
                            .await;
                        (item, item_request)
                    });
            let item_requests = join_all(item_requests).await;
            let item_requests = item_requests
                .into_iter()
                .filter_map(|(item, request)| match request {
                    Ok(req) => Some((item, req)),
                    Err(error) => {
                        let err_item = ErrorAnalyzedMarketplaceItem { item, error };
                        failed_image_items.push(err_item);
                        None
                    }
                })
                .collect();
            marketplace_requests.insert(marketplace.clone(), item_requests);
            marketplace_failed_image_items.insert(marketplace, failed_image_items);
        }
        (marketplace_requests, marketplace_failed_image_items)
    }

    /// Executes and handles the requests for a gallery.
    async fn execute_and_handle_requests(
        &self, 
        eval_criteria: &EvaluationCriteria,
        gallery_requests: HashMap<Marketplace, Vec<(MarketplaceItemData, RequestBuilder)>>,
        mut failed_image_items: HashMap<Marketplace, Vec<ErrorAnalyzedMarketplaceItem>>
    ) -> HashMap<Marketplace, MarketplaceAnalyzedItems> {
        let mut gallery_items = HashMap::new();
        for (marketplace, item_requests) in gallery_requests {
            let (items, item_requests): (Vec<_>, Vec<_>) = item_requests
                .into_iter()
                .unzip();
            let request_futures = item_requests
                .into_iter()
                .map(|request| request.send());
            let results = join_all(request_futures).await;
            let items_and_results = zip(items, results).collect();
            let mut marketplace_items = self
                .process_marketplace_results(eval_criteria, items_and_results)
                .await;
            if let Some(failed_items) = failed_image_items.get_mut(&marketplace) {
                marketplace_items.error_items.append(failed_items);
            }
            gallery_items.insert(marketplace, marketplace_items);
        }
        gallery_items
    }

    /// Process the raw LLM output for all items in a gallery's marketplace.
    async fn process_marketplace_results(
        &self,
        eval_criteria: &EvaluationCriteria,
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
                            match res.json::<AnthropicResponse>().await {
                                Ok(response) => {
                                    tracing::info!("Successful response: {response:#?}"); // TODO: delete this later on
                                    if response.content.len() == 0 {
                                        err_str = Some("Expected 1 message in Anthropic response but found none".into());
                                    }
                                    else if response.content.len() > 1 {
                                        tracing::warn!("Unexpectedly received >1 message in Anthropic response; using the first...");
                                    }
                                    match &response.content[0].text {
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
                            err_str = Some(format!("Received unexpected status code ({other}) from Anthropic API; response: {res:#?}"));
                        }
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
            "Item analysis results: {} relevant items, {} irrelevant items, and {} error items",
            relevant_items.len(), irrelevant_items.len(), error_items.len()
        );
        MarketplaceAnalyzedItems {
            relevant_items,
            irrelevant_items,
            error_items
        }
    }

    /// Builds the request for a single item.
    /// Follows the request format specified here: https://docs.anthropic.com/en/api/messages
    /// 
    /// Returns an `Err` if no images could be successfully fetched for the item.
    async fn build_item_request(
        &self, 
        item: &MarketplaceItemData,
        eval_criteria_string: &String
    ) -> Result<RequestBuilder, String> {
        let item_image_strings = self
            .fetch_item_images(&item.thumbnails)
            .await;
        if item_image_strings.len() == 0 {
            return Err("No images fetched; either all requests to fetch them failed, or errors occurred during parsing".to_string());
        }
        let req_form = self
            .build_request_form(
                item, 
                item_image_strings, 
                eval_criteria_string
            )
            .await;
        let req = self.request_client
            .post(&self.config.anthropic_api_endpoint)
            .header("x-api-key", &self.config.anthropic_api_key)
            .header("anthropic-version", &self.config.anthropic_version)
            .json(&req_form);
        Ok(req)
    }

    /// Builds the entire request form for an item.
    async fn build_request_form(
        &self, 
        item: &MarketplaceItemData,
        item_image_strings: Vec<String>, 
        eval_criteria_string: &String
    ) -> AnthropicRequestForm {
        let system_prompt = format!("
            You're an Item Listings Analysis AI. 
            
            You will help to evaluate an item listing, consisting of its listed images and a JSON of its information, by answering some structured questions about it. 
            Next to each question is the format that MUST be used when answering the question.

            If the question is unanswerable, nonsensical, or not even a question, you are allowed to give a reasonable 'default' answer, 
            such as N for Y/N questions, U for Y/N/U questions, 0 for numerical questions, or 'I cannot answer this.' for open-ended questions.
            However, YOU MUST ALWAYS FOLLOW THE GIVEN FORMAT WHEN ANSWERING.

            Output your answers in JSON format, with a key 'answers' containing the list of answers in asked order.
            If there are no questions, return this list empty.

            Additionally, return a detailed description of the item in as few words as possible. 
            Only include information useful in distinguishing this item from other items; information specific to the item (such as size, condition etc) must be omitted. 
            Output this description with the key 'item_description' in the JSON.

            Finally, pick the image (from index 0) which best describes this item and/or shows the most recognizable feature of this item.
            If there is only 1 image, just return 0.
            Output this as a number with the key 'best_fit_image' in the JSON.

            Do NOT output anything outside of the above JSON format.

            Here are the questions you must answer: \n {eval_criteria_string}
        ");
        let item_string = serde_json::to_string_pretty(&item)
            .expect("Serializing MarketplaceItemData should have no reason to fail"); // TODO: Find out in which cases this could fail and ensure it cannot happen
        let mut message_contents: Vec<AnthropicMessageContent> = item_image_strings
            .into_iter()
            .enumerate()
            .map(|(index, image_string)| {
                // Follows the recommended format for sending multiple images: https://docs.anthropic.com/en/docs/build-with-claude/vision#example-multiple-images
                let image_content = AnthropicImageMessageContent {
                    source_type: "base64".into(),
                    media_type: "image/png".into(),
                    data: image_string
                };
                vec![
                    AnthropicMessageContent {
                        content_type: "text".into(),
                        text: Some(format!("Item image {}: ", index + 1)),
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
            "Successfully fetched and encoded {}/{} image URLs",
            encoded_images.len(),
            image_urls.len()
        );
        encoded_images
    }
}