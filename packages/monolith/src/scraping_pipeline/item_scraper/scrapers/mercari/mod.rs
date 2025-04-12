use std::error::Error;

use futures::future::join_all;
use reqwest::{Client, RequestBuilder};
use types::{MercariItemData, MercariItemResponse};
use crate::{domain::{domain_types::ItemId, item_data::MarketplaceItemData}, utils::generate_dpop::generate_dpop};

const REQ_URL: &str = "https://api.mercari.jp/items/get"; // TODO: move to config

mod types;

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
        let dpop_key = match generate_dpop(&REQ_URL, "GET") {
            Ok(key) => key,
            Err(err) => return vec![Err(err)]
        };
        tracing::debug!("Generated dpop key: {dpop_key}");
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
            .header("x-platform", "web") 
            .header("accept", "application/json")
    }    

    /// Handle the raw responses from the item scrape. 
    async fn handle_responses(&self, responses: Vec<(ItemId, Result<reqwest::Response, reqwest::Error>)>) -> Vec<Result<MarketplaceItemData, String>> {
        let parsed_response_futures = responses.into_iter()
            .map(|(id, response)| async move {
                match response {
                    Ok(res) => {
                        match res.error_for_status() {
                            Ok(res) => {
                                match res.json::<MercariItemResponse>().await {
                                    Ok(data) => {
                                        let item = self.map_to_marketplace_item(data.data);
                                        Ok(item)
                                    },
                                    Err(err) => Err(format!("Error deserializing item data: {err} (source: {:?})", err.source())),
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

    /// Map from Mercari's raw data to the internal type.
    fn map_to_marketplace_item(&self, data: MercariItemData) -> MarketplaceItemData {
        MarketplaceItemData {
            item_id: data.id.into(),
            name: data.name,
            price: data.price.into(),
            description: data.description,
            status: data.status.into(),
            created: data.created.into(),
            seller_id: data.seller.id.to_string(),
            category: data.item_category.name,
            thumbnails: data.thumbnails,
            item_condition: data.item_condition.name,
            updated: data.updated.into()
        }
    }
}
