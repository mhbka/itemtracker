//! Types for modeling the response from Mercari Item API.

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(super) struct MercariItemResponse {
    pub result: String,
    pub data: MercariItemData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(super) struct MercariItemData {
    pub id: String,
    pub name: String,
    pub price: f32,
    pub description: String,
    pub status: String,
    pub seller: Seller,
    pub photos: Vec<String>,
    pub thumbnails: Vec<String>,
    pub item_category: ItemCategory,
    pub item_condition: ItemCondition,
    pub item_brand: Option<ItemBrand>,
    pub created: i64,
    pub updated: i64,
    pub num_likes: usize,
    pub num_comments: usize,
    pub comments: Vec<Comment>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(super) struct Seller {
    pub id: usize,
    pub name: String,
    pub photo_thumbnail_url: String,
    pub num_sell_items: usize,
    pub ratings: Ratings,
    pub num_ratings: usize,
    pub score: usize,
    pub star_rating_score: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(super) struct Ratings {
    pub good: usize,
    pub normal: usize,
    pub bad: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(super) struct ItemCategory {
    pub id: usize,
    pub name: String,
    pub parent_category_id: usize,
    pub parent_category_name: String,
    pub root_category_id: usize,
    pub root_category_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(super) struct ItemCondition {
    pub id: usize,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(super) struct ItemBrand {
    pub id: usize,
    pub name: String,
    pub sub_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(super) struct Comment {
    pub id: usize,
    pub user: CommentUser,
    pub message: String,
    pub created: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(super) struct CommentUser {
    pub id: usize,
    pub name: String,
    pub photo_thumbnail_url: String,
}