use chrono::NaiveDateTime;

/// Model of a scraped item.
pub struct ItemModel {
    pub marketplace: String,
    pub item_id: String,
    pub name: String,
    pub price: f32,  
    pub description: String,
    pub status: String,
    pub category: String,
    pub thumbnails: Vec<String>, 
    pub item_condition: String,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
    pub seller_id: String,
    pub seller_name: String,
}