use std::error::Error;

use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::galleries::domain_types::UnixUtcDateTime;
use crate::galleries::search_criteria::GallerySearchCriteria;
use crate::galleries::domain_types::ItemId;
use crate::utils::generate_dpop::generate_dpop;

const REQ_URL: &str = "https://api.mercari.jp/v2/entities:search";

pub(super) struct MercariSearchScraper {
    client: Client
}

impl MercariSearchScraper {
    /// Instantiate the scraper.
    pub(super) fn new() -> Self {
        Self {
            client: Client::new()
        }
    }

    /// Performs the search scrape for Mercari.
    pub(super) async fn request(
        &self, 
        search_criteria: &GallerySearchCriteria,
        previous_scraped_item_datetime: UnixUtcDateTime
    ) -> Result<Vec<ItemId>, String> {
        let dpop_key = match generate_dpop(&REQ_URL, "POST") {
            Ok(key) => {
                tracing::trace!("Generated dpop key: {key}");
                key
            },
            Err(err) => {
                tracing::warn!("Failed to generate dpop key (this should not happen)");
                return Err(err);
            }
        };
        let mut item_ids = vec![];
        let mut next_page_token = "".to_string();
        loop { // keep requesting new pages of search; break only when `scraped_next_page_token` is None
            let request = self.build_request(
                &dpop_key, 
                search_criteria, 
                &next_page_token
            );
            let response = request.send().await;
            match self.handle_response(&previous_scraped_item_datetime, response).await {
                Ok((scraped_item_ids, scraped_next_page_token)) => {
                    item_ids.extend_from_slice(&scraped_item_ids);
                    match scraped_next_page_token {
                        Some(token) => next_page_token = token,
                        None => break
                    }
                },
                Err(err) => return Err(err)
            }
        };
        Ok(item_ids)
    }

    /// Handle the raw response from the search scrape.
    /// 
    /// Returns the item IDs in the response + an optional string containing the next page token;
    /// if present, the next page should continue to be scraped as well.
    /// 
    /// Returns an `Err` if the response had an error.
    async fn handle_response(
        &self, 
        previous_scraped_item_datetime: &UnixUtcDateTime,
        response: Result<reqwest::Response, reqwest::Error>
    ) -> Result<(Vec<ItemId>, Option<String>), String> {
        match response {
            Ok(res) => {
                match res.error_for_status() {
                    Ok(res) => {
                        match res.json::<MercariSearchData>().await {
                            Ok(res) => {
                                let mut items = res.items.into_iter();
                                match items.all(|item| &item.updated > previous_scraped_item_datetime) {
                                    true => {
                                        let item_ids = items
                                            .map(|item| item.id.into())
                                            .collect();
                                        let next_page_token = match res.meta.next_page_token.as_ref() {
                                            "" => None,
                                            _ => Some(res.meta.next_page_token)
                                        };
                                        return Ok((item_ids, next_page_token));
                                    },
                                    false => {
                                        let item_ids = items
                                            .filter(|item| &item.updated > previous_scraped_item_datetime)
                                            .map(|item| item.id.into())
                                            .collect();
                                        return Ok((item_ids, None));
                                    }
                                }
                            },
                            Err(err) => Err(format!("Error deserializing scraped search data:\n {err}\n (source: {:?})", err.source())),
                        }
                    },
                    Err(err) => Err(format!("Error code while scraping search:\n {err}"))
                }
            },
            Err(err) => Err(format!("Error scraping search: {err}"))
        }
    }

    /// Build the request for scraping the search.
    fn build_request(
        &self, 
        dpop_key: &String,
        search_criteria: &GallerySearchCriteria, 
        next_page_token: &str
    ) -> RequestBuilder {
        self.client
            .post(REQ_URL)
            .json(&self.build_payload(search_criteria, next_page_token))
            .header("dpop", dpop_key)
            .header("x-platform", "web") // TODO: is this necessary
    }

    /// Build the payload for scraping the search.
    fn build_payload(&self, search_criteria: &GallerySearchCriteria, next_page_token: &str) -> Value {
        json!(
            {
                "userId": "",
                "pageSize": 120,
                "pageToken": next_page_token,
                "searchSessionId": "adc97d31b66ba64443fe25778dee77c2",
                "indexRouting": "INDEX_ROUTING_UNSPECIFIED",
                "thumbnailTypes": [],
                "searchCondition": {
                    "keyword": search_criteria.keyword,
                    "excludeKeyword": search_criteria.exclude_keyword,
                    "sort": "SORT_CREATED_TIME",
                    "order": "ORDER_DESC", 
                    "status": [], // TODO: add this to search criteria?
                    "sizeId": [],
                    "categoryId": [], // TODO: add this to search criteria?
                    "brandId": [],
                    "sellerId": [],
                    "priceMin": 0,
                    "priceMax": 0,
                    "itemConditionId": [],
                    "shippingPayerId": [],
                    "shippingFromArea": [],
                    "shippingMethod": [],
                    "colorId": [],
                    "hasCoupon": false,
                    "attributes": [],
                    "itemTypes": [],
                    "skuIds": [],
                    "shopIds": []
                },
                "serviceFrom": "suruga",
                "withItemBrand": true,
                "withItemSize": false,
                "withItemPromotions": true,
                "withItemSizes": true,
                "withShopname": false,
                "useDynamicAttribute": true,
                "withSuggestedItems": true,
                "withOfferPricePromotion": false,
                "withProductSuggest": true,
                "withParentProducts": false,
                "withProductArticles": false,
                "withSearchConditionId": false,
                "withAuction": false,
            }
        )
    }
}

/// The data returned from a search scrape.
/// 
/// Note: Other values are returned than what is here, but we only deserialize whatever we need.
#[derive(Serialize, Deserialize)]
struct MercariSearchData {
    pub items: Vec<MercariSearchItemData>,
    pub meta: MercariSearchMetadata
}

/// Represents a single item's data from the search scrape.
/// 
/// Note: Other values are returned than what is here, but we only deserialize whatever we need.
#[derive(Serialize, Deserialize)]
struct MercariSearchItemData {
    pub id: String,
    pub updated: UnixUtcDateTime
}

/// Metadata returned in a search scrape.
///
/// Note: Other values are returned than what is here, but we only deserialize whatever we need.
#[derive(Serialize, Deserialize)]
struct MercariSearchMetadata {
    #[serde(rename(deserialize = "nextPageToken"))]
    pub next_page_token: String
}