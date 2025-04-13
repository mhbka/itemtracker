use chrono::NaiveDateTime;
use diesel::prelude::*;
use crate::{domain::{domain_types::Marketplace, item_data::MarketplaceItemData}, schema::marketplace_items};

/// Model of a marketplace item.
#[derive(Queryable, Identifiable, Debug)]
#[table_name = "marketplace_items"]
pub struct ItemModel {
    pub id: i32,
    pub marketplace: String,
    pub item_id: String,
    pub name: String,
    pub price: f64,  
    pub description: String,
    pub status: String,
    pub category: String,
    pub thumbnails: Vec<String>, 
    pub item_condition: String,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
    pub seller_id: String,
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