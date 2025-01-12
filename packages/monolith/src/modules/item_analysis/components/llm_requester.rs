use futures::future::{join, join_all};
use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};

use crate::{config::ItemAnalysisConfig, galleries::{domain_types::Marketplace, eval_criteria::EvaluationCriteria, items::{item_data::MarketplaceItemData, pipeline_items::ScrapedItems}, scraping_pipeline::GalleryScrapedState}, messages::message_types::item_analysis::ItemAnalysisError};

/// Orchestrates requesting of the LLM for a gallery's items.
pub(in crate::modules::item_analysis) struct LLMRequester {
    config: ItemAnalysisConfig,
    request_client: Client,
}

impl LLMRequester {
    /// Initialize the requester.
    pub fn new(config: ItemAnalysisConfig) -> Self {
        Self { 
            config,
            request_client: Client::new()
        }
    }

    /// Carries out requesting the LLM for a gallery.
    pub async fn request_gallery(&mut self, gallery: GalleryScrapedState) -> Result<(), ItemAnalysisError> {
        let gallery_id = gallery.gallery_id;
        let items = gallery.items;
        let eval_criteria_string = gallery.evaluation_criteria.describe_criteria();
        
        let gallery_requests = self.build_requests(items.clone(), eval_criteria_string);
        for (marketplace, requests) in gallery_requests {
            
        }
        
        Ok(())
    }

    /// Builds the requests to the LLM for all the gallery's items.
    fn build_requests(
        &self, 
        items: ScrapedItems,
        eval_criteria_string: String
    ) -> Vec<(Marketplace, Vec<RequestBuilder>)> {
        items.marketplace_items
            .into_iter()
            .map(|(marketplace, items)| {
                let item_requests = items
                    .into_iter()
                    .map(|item| self.build_item_request(item, &eval_criteria_string))
                    .collect();
                (marketplace, item_requests)
            })
            .collect()
    }

    /// Executes and handles the requests for a gallery.
    async fn execute_requests(&self, gallery_requests: Vec<(Marketplace, Vec<RequestBuilder>)>) {
        for (marketplace, item_requests) in gallery_requests {
            let request_futures = item_requests
                .into_iter()
                .map(|request| request.send());
            let results = join_all(request_futures).await;
            for res in results {
                match res {
                    Ok(res) => {
                        match res.json::<AnthropicResponse>().await {
                            Ok(response) => {

                            },
                            Err(err) => {}
                        }
                    },
                    Err(err) => {}
                }
            }
        }
    }

    /// Builds the request for a single item.
    /// 
    /// Follows the request format specified here: https://docs.anthropic.com/en/api/messages
    fn build_item_request(
        &self, 
        item: MarketplaceItemData,
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
        item: MarketplaceItemData, 
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

/// The request form for querying Anthropic API.
/// 
/// **NOTE**: There are other optional parameters, but they're left out as we don't (currently) use them.
/// Check the docs for what they are.
/// 
/// https://docs.anthropic.com/en/api/messages
#[derive(Serialize, Deserialize, Clone, Debug)]
struct AnthropicRequestForm {
    model: String,
    max_tokens: usize,
    messages: Vec<AnthropicMessage>,
    system: String
}

/// A single message in the content of a Anthropic API request.
/// 
/// https://docs.anthropic.com/en/api/messages
#[derive(Serialize, Deserialize, Clone, Debug)]
struct AnthropicMessage {
    role: String,
    content: String
}

/// The response received from a Anthropic API request.
/// 
/// **NOTE**: There is other data returned in the response, but we don't (currently) use it,
/// so we just leave them out. Check the docs for what they are.
/// 
/// https://docs.anthropic.com/en/api/messages
#[derive(Serialize, Deserialize, Clone, Debug)]
struct AnthropicResponse {
    id: String,
    content: Vec<AnthropicResponseContent>,
    usage: AnthropicUsage
}

/// The actual response from the LLM.
/// 
/// https://docs.anthropic.com/en/api/messages
#[derive(Serialize, Deserialize, Clone, Debug)]
struct AnthropicResponseContent {
    text: String,
    #[serde(alias = "type")] // can't directly name a struct member `type` as it's a keyword
    content_type: String
}

/// The usage data for this query.
/// 
/// https://docs.anthropic.com/en/api/messages
#[derive(Serialize, Deserialize, Clone, Debug)]
struct AnthropicUsage {
    input_tokens: usize,
    output_tokens: usize
}
