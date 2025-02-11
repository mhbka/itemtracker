use futures::future::join_all;
use reqwest::{Client, RequestBuilder};
use crate::{galleries::{domain_types::ItemId, items::item_data::MarketplaceItemData}, modules::scraper::components::utils::generate_dpop::generate_dpop};

const REQ_URL: &str = "https://api.mercari.jp/items/get"; // TODO: move to config

/// This struct is in charge of scraping items from Mercari.
pub(super) struct MercariItemScraper {
    client: Client
}

impl MercariItemScraper {
    pub fn new() -> Self {
        Self {
            client: Client::new()
        }
    }

    /// Performs the item scraping for Mercari.
    pub async fn request(&self, item_ids: Vec<ItemId>) -> Vec<Result<MarketplaceItemData, String>> {
        let dpop_key = match generate_dpop(&REQ_URL, "get") {
            Ok(key) => key,
            Err(err) => return vec![Err(err)]
        };
        let request_futures = item_ids
            .into_iter()
            .map(|id| {
            let dpop_key = dpop_key.clone();
            async move {
                let req = self
                    .create_request(&dpop_key, &id)
                    .send()
                    .await;
                (id, req)
            }
            });
        let responses = join_all(request_futures).await;
        self.handle_responses(responses).await
    }

    /// Create the request for an item ID.
    fn create_request(&self, dpop_key: &String, item_id: &ItemId) -> RequestBuilder {
        self.client
            .get(format!("{REQ_URL}?id={item_id}"))
            .header("dpop", dpop_key)
            .header("x-platform", "web") // TODO: see if these 2 are actually necessary
            .header("content-type", "application/json")
    }    

    /// Handle the raw responses from the item scrape. 
    async fn handle_responses(&self, responses: Vec<(ItemId, Result<reqwest::Response, reqwest::Error>)>) -> Vec<Result<MarketplaceItemData, String>> {
        let parsed_response_futures = responses.into_iter()
            .map(|(id, response)| async move {
                match response {
                    Ok(res) => {
                        match res.error_for_status() {
                            Ok(res) => {
                                match res.json::<MarketplaceItemData>().await {
                                    Ok(data) => Ok(data),
                                    Err(err) => Err(format!("Error deserializing item data: {err}")), // TODO: print out the actual data here?
                                }
                            },
                            Err(err) => Err(format!("Error code while requesting for item {id}: {err}")),
                        }
                    },
                    Err(err) => Err(format!("Error requesting for item {id}: {err}")),
                }   
            });
        join_all(parsed_response_futures).await
    }
}