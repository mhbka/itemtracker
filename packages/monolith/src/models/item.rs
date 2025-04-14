use chrono::NaiveDateTime;
use diesel::{prelude::*, pg::Pg};
use crate::{domain::{domain_types::{ItemId, Marketplace, UnixUtcDateTime}, item_data::MarketplaceItemData}, schema::marketplace_items};

/// Model of a marketplace item.
#[derive(Queryable, Selectable, Identifiable, Debug)]
#[diesel(check_for_backend(Pg))]
#[diesel(table_name = marketplace_items)]
pub struct ItemModel {
    pub id: i32,
    pub marketplace: String,
    pub item_id: String,
    pub name: String,
    pub price: f64,  
    pub description: String,
    pub status: String,
    pub category: String,
    pub thumbnails: Vec<Option<String>>, 
    pub item_condition: String,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
    pub seller_id: String,
}

impl ItemModel {
    /// Convert to the domain type.
    pub fn convert_to(self) -> MarketplaceItemData {
        let thumbnails = self.thumbnails
            .into_iter()
            .filter_map(|t| t)
            .collect();

        MarketplaceItemData {
            item_id: ItemId::from(self.item_id),
            name: self.name,
            price: self.price,
            description: self.description,
            status: self.status,
            thumbnails,
            seller_id: self.seller_id,
            category: self.category,
            item_condition: self.item_condition,
            created: UnixUtcDateTime::new(self.created.and_utc()),
            updated: UnixUtcDateTime::new(self.updated.and_utc())
        }
    }
}

/// For inserting a new item.
/// 
/// ### NOTE
/// Fields mapped 1:1 to the domain type are borrowed, hence the lifetime.
/// All none-borrowed fields are original, cannot be mapped 1:1, or are `Copy`.
#[derive(Insertable, Debug)]
#[table_name = "marketplace_items"]
pub struct NewItem<'a> {
    pub marketplace: String,
    pub item_id: &'a str,
    pub name: &'a str,
    pub price: f64,  
    pub description: &'a str,
    pub status: &'a str,
    pub category: &'a str,
    pub thumbnails: &'a Vec<String>, 
    pub item_condition: &'a str,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
    pub seller_id: &'a str,
}

impl<'a> NewItem<'a> {
    /// Convert from the domain type.
    pub fn convert(marketplace: Marketplace, item: &'a MarketplaceItemData) -> Self {
        Self {
            marketplace: marketplace.to_string(),
            item_id: &item.item_id,
            name: &item.name,
            price: item.price,
            description: &item.description,
            status: &item.status,
            category: &item.category,
            thumbnails: &item.thumbnails,
            item_condition: &item.item_condition,
            created: item.created.naive_utc(),
            updated: item.updated.naive_utc(),
            seller_id: &item.seller_id,
        }
    }
}