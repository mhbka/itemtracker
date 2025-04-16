use crate::{domain::{eval_criteria::CriterionAnswer, item_data::MarketplaceItemData, pipeline_items::EmbeddedMarketplaceItem}, models::{gallery_session::GallerySessionModel, item::ItemModel}, schema::*, utils::vec::OnlySome};
use diesel::{pg::Pg, Associations, Identifiable, Insertable, Queryable, Selectable};

/// Model of the `embedded_marketplace_items` table.
#[derive(Queryable, Identifiable, Selectable, Associations, Debug, Clone)]
#[diesel(belongs_to(ItemModel, foreign_key = marketplace_item_id))]
#[diesel(belongs_to(GallerySessionModel, foreign_key = gallery_session_id))]
#[diesel(check_for_backend(Pg))]
#[diesel(table_name = embedded_marketplace_items)]
pub struct EmbeddedItemModel {
    pub id: i32,
    /// **NOTE**: This refers to the `id` of the marketplace item,
    /// not `item_id` (which is its ID in the marketplace itself).
    pub marketplace_item_id: i32,
    pub gallery_session_id: i32,
    pub item_description: String,
    pub description_embedding: Vec<Option<f32>>,
    pub image_embedding: Vec<Option<f32>>,
    pub evaluation_answers: Vec<Option<CriterionAnswer>>,
}

impl EmbeddedItemModel {
    /// Convert to the domain type.
    pub fn convert_to(self, item: MarketplaceItemData) -> EmbeddedMarketplaceItem {
        EmbeddedMarketplaceItem {
            item,
            evaluation_answers: self.evaluation_answers.only_some(),
            item_description: self.item_description,
            description_embedding: self.description_embedding.only_some(),
            image_embedding: self.image_embedding.only_some()
        }
    }
}

/// For inserting a new embedded item.
/// 
/// **TODO**: changed actual model to Options so see if this must be changed too 
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