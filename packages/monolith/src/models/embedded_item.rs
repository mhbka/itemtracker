use crate::{domain::{eval_criteria::CriterionAnswer, item_data::MarketplaceItemData, pipeline_items::EmbeddedMarketplaceItem}, models::{gallery_session::GallerySessionModel, item::ItemModel}, schema::*};
use diesel::{Insertable, Queryable, Identifiable, Associations};

/// Model of the `embedded_marketplace_items` table.
#[derive(Queryable, Identifiable, Associations, Debug, Clone)]
#[belongs_to(ItemModel, foreign_key = "marketplace_item_id")]
#[belongs_to(GallerySessionModel, foreign_key = "gallery_session_id")]
#[table_name = "embedded_marketplace_items"]
pub struct EmbeddedItemModel {
    pub id: i32,
    pub marketplace_item_id: i32,
    pub gallery_session_id: i32,
    pub item_description: String,
    pub description_embedding: Vec<f32>,
    pub image_embedding: Vec<f32>,
    pub evaluation_answers: Vec<CriterionAnswer>,
}

/// For inserting a new embedded item.
#[derive(Insertable, Debug, Clone)]
#[table_name = "embedded_marketplace_items"]
pub struct NewEmbeddedMarketplaceItem<'a> {
    pub marketplace_item_id: i32,
    pub gallery_session_id: i32,
    pub item_description: &'a str,
    pub description_embedding: &'a Vec<f32>,
    pub image_embedding: &'a Vec<f32>,
    pub evaluation_answers: &'a Vec<CriterionAnswer>, 
}

impl<'a> NewEmbeddedMarketplaceItem<'a> {
    /// Convert from the domain type.
    /// 
    /// Requires that the item data and gallery session already be inserted, 
    /// so that their IDs can be set as `marketplace_item_id` and `gallery_session_id`.
    pub fn convert(
        marketplace_item_id: i32, 
        gallery_session_id: i32, 
        item: &'a EmbeddedMarketplaceItem
    ) -> Self {
        Self {
            marketplace_item_id,
            gallery_session_id,
            item_description: &item.item_description,
            description_embedding: &item.description_embedding,
            image_embedding: &item.image_embedding,
            evaluation_answers: &item.evaluation_answers
        }
    }
}