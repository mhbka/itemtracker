use std::{collections::HashMap, error::Error, io::Cursor};
use image::DynamicImage;
use reqwest::{multipart::{self, Part}, Client, RequestBuilder};
use serde::{Deserialize, Serialize};

use crate::{config::ItemEmbedderConfig, domain::{domain_types::{GalleryId, Marketplace}, pipeline_items::{AnalyzedMarketplaceItem, EmbeddedMarketplaceItem, ErrorEmbeddedMarketplaceItem, MarketplaceAnalyzedItems, MarketplaceEmbeddedAndAnalyzedItems}}};

/// The response from the embedder.
/// 
/// Each embedding should be in the same order as was sent,
/// and the number of embeddings must equal the number of item text/images sent.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct EmbedderResponse {
    text_embeddings: Vec<Vec<f32>>,
    image_embeddings: Vec<Vec<f32>>,
}


/// In charge of handling requests to the actual embedding service.
#[derive(Clone)]
pub(super) struct Embedder {
    config: ItemEmbedderConfig,
    request_client: Client
}

impl Embedder {
    /// Initialize the struct.
    pub fn new(config: &ItemEmbedderConfig) -> Self {
        Self {
            config: config.clone(),
            request_client: Client::new()
        }
    }

    /// Embed a gallery's items' description and chosen images.
    pub async fn embed_gallery(
        &mut self, 
        gallery_id: GalleryId, 
        items: HashMap<Marketplace, MarketplaceAnalyzedItems>
    ) -> HashMap<Marketplace, MarketplaceEmbeddedAndAnalyzedItems> {
        let mut embedded_items = HashMap::new();
        for (marketplace, items) in items {
            let (
                request,
                valid_items,
                failed_to_fetch_image_items
            ) = self.build_marketplace_request(items.relevant_items).await;
            let marketplace_items = match self.execute_and_handle_request(request, valid_items).await {
                Ok(marketplace_embedded_items) => {
                    MarketplaceEmbeddedAndAnalyzedItems {
                        embedded_items: marketplace_embedded_items,
                        irrelevant_analyzed_items: items.irrelevant_items,
                        error_analyzed_items: items.error_items,
                        error_embedded_items: Vec::new()
                    }
                },
                Err((error_valid_items, err)) => {
                    tracing::info!("Marketplace {marketplace} for gallery {gallery_id} got an error during embedding: {err}");
                    let error_items = error_valid_items
                        .into_iter()
                        .map(|item| ErrorEmbeddedMarketplaceItem { item, error: err.clone() })
                        .collect();
                    MarketplaceEmbeddedAndAnalyzedItems {
                        embedded_items: Vec::new(),
                        irrelevant_analyzed_items: items.irrelevant_items,
                        error_analyzed_items: items.error_items,
                        error_embedded_items: error_items
                    }
                }
            };
            embedded_items.insert(marketplace, marketplace_items);
        }   
        embedded_items
    }

    /// Builds the request for items under a marketplace.
    /// 
    /// Returns:
    /// - The request
    /// - The items for which the request was successfully built
    /// - Items which encountered errors during image fetching (as `ErrorEmbeddedMarketplaceItem`)
    async fn build_marketplace_request(&mut self, items: Vec<AnalyzedMarketplaceItem>) 
    -> (RequestBuilder, Vec<AnalyzedMarketplaceItem>, Vec<ErrorEmbeddedMarketplaceItem>) {
        let mut form = multipart::Form::new();
        let mut failed_items = Vec::new();
        let mut valid_items_and_images = Vec::new();

        // fetch item images; for failed items, push them into `failed_items`
        for item in items {
            match self
                .get_item_image(&item.item.thumbnails, item.best_fit_image)
                .await {
                    Ok(item_image) => valid_items_and_images.push((item, item_image)),
                    Err(error) => {
                        let err_item = ErrorEmbeddedMarketplaceItem { item, error };
                        failed_items.push(err_item);
                    }
                }
        }

        // NOTE: we add parts to the form in same order of the valid items; the embedder will return embeddings in the same order
        // NOTE 2: items with images that failed to write, will also be pushed into `failed_items`
        let mut valid_items = Vec::new();
        for (index, (item, item_image)) in valid_items_and_images.into_iter().enumerate() {
            let mut image_bytes = Cursor::new(Vec::new());
            match item_image.write_to(&mut image_bytes, image::ImageFormat::Png) {
                Ok(_) => {
                    let image_part = Part::bytes(image_bytes.into_inner())
                        .file_name(format!("image{index}"));
                    let text_part = Part::text(item.item_description.clone());
                    form = form
                        .part("image", image_part)
                        .part("text", text_part);
                    valid_items.push(item);
                },
                Err(err) => {
                    let err_item = ErrorEmbeddedMarketplaceItem { item, error: err.to_string() };
                    failed_items.push(err_item);
                }
            }
        }
        let request = self.request_client
            .post(&self.config.embedder_endpoint)
            .multipart(form);
        (request, valid_items, failed_items)
    }

    /// Executes and handles the request for a marketplace.
    /// 
    /// Returns an `Err` with the original valid items and an error string, 
    /// if an unrecoverable error occurs while requesting or response parsing.
    async fn execute_and_handle_request(
        &mut self, 
        request: RequestBuilder,
        valid_items: Vec<AnalyzedMarketplaceItem>
    ) -> Result<Vec<EmbeddedMarketplaceItem>, (Vec<AnalyzedMarketplaceItem>, String)> {
        match request.send().await {
            Ok(res) => {
                match res.error_for_status() {
                    Ok(res) => {
                        match res.json::<EmbedderResponse>().await {
                            Ok(res) => {
                                if res.text_embeddings.len() != valid_items.len() || res.image_embeddings.len() != valid_items.len() {
                                    let err_str = format!(
                                        "Number of text/image embeddings ({}/{}) doesn't match number of valid items ({})",
                                        res.text_embeddings.len(), res.image_embeddings.len(), valid_items.len()
                                    );
                                    return Err((valid_items, err_str));
                                }
                                let embedded_items = valid_items
                                    .into_iter()
                                    .zip(res.text_embeddings.into_iter())
                                    .zip(res.image_embeddings.into_iter())
                                    .map(|((item, text_embedding), image_embedding)| 
                                        EmbeddedMarketplaceItem {
                                            item: item.item,
                                            evaluation_answers: item.evaluation_answers,
                                            item_description: item.item_description,
                                            description_embedding: text_embedding,
                                            image_embedding
                                        }
                                    )
                                    .collect();
                                Ok(embedded_items)
                            },
                            Err(err) => {
                                let err_str = format!("Could not parse response into EmbedderResponse: {err} (source: {:?})", err.source());
                                Err((valid_items, err_str))
                            }
                        }
                    },
                    Err(err) => {
                        let err_str = format!("Received an error status: {err} (source: {:?})", err.source());
                        Err((valid_items, err_str))
                    }
                }
            },
            Err(err) =>{ 
                let err_str = format!("Received an error from the embedder: {err} (source: {:?})", err.source());
                Err((valid_items, err_str))
            }
        }
    }

    /// Fetches the image pointed by the item's `best_fit_image`.
    /// If this number is invalid for some reason, fetches the first image.
    /// 
    /// Returns an `Err` if the image couldn't be fetched, or some error occurred during its parsing.
    async fn get_item_image(
        &mut self, 
        image_urls: &Vec<String>,
        best_fit_image: usize
    ) -> Result<DynamicImage, String> {
        let chosen_image_url = match image_urls.get(best_fit_image) {
            Some(url) => url,
            None => match image_urls.get(0) {
                Some(url) => url,
                None => return Err("Item doesn't contain any image URLs".to_string())
            }
        };
        match self.request_client
            .get(chosen_image_url)
            .send()
            .await {
                Ok(res) => {
                    match res.bytes().await {
                        Ok(bytes) => {
                            match image::load_from_memory(&bytes) {
                                Ok(image) => Ok(image),
                                Err(err) => Err(format!("Failed to decode fetched image URL bytes into an image: {err}"))
                            }
                        },
                        Err(err) => Err(format!("Failed to decode fetched image URL into bytes: {err}"))
                    }
                },
                Err(err) => Err(format!("Failed to fetch an image URL: {err}"))
            }
    }
}