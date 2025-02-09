use futures::future::join_all;
use reqwest::Client;
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

    /// Attempt to scrape a list of item IDs,
    /// returning a list with (in order) the item's data, or an `Err` if the scrape wasn't successful.
    /// 
    /// Returns the `Vec` with a single `Err` if the *dpop* key generation was unsuccessful (should never happen).
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
                self.client
                    .get(format!("{REQ_URL}?id={id}"))
                    .header("dpop", dpop_key)
                    .header("x-platform", "web") // TODO: see if these 2 are actually necessary
                    .header("content-type", "application/json")
                    .send()
                    .await
            }
            });
        let responses = join_all(request_futures).await;
        self.handle_responses(responses)
    }    

    /// Parses the raw responses from the item scrape. 
    fn handle_responses(&self, responses: Vec<Result<reqwest::Response, reqwest::Error>>) -> Vec<Result<MarketplaceItemData, String>> {
        responses.into_iter()
            .map(|response| {
                match response {
                    Ok(res) => {

                    },
                    Err(err) =>  {
                        
                    }
                }   
            })
            .collect()
    }
}