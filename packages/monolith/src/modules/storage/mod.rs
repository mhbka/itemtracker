//! This module contains actual sub-modules for interfacing with different areas of storage.
//! 
//! Note that there is no "global" storage module;
//! sub-modules are simply re-exported here.

pub mod marketplace_items;
pub mod item_image_embeddings;

pub use marketplace_items::MarketplaceItemsModule;
pub use item_image_embeddings::ItemImageEmbeddingsModule;