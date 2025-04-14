use diesel_async::{pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager}, AsyncPgConnection};
use galleries::GalleryStore;
use gallery_sessions::GallerySessionsStore;
use crate::config::StoreConfig;

pub mod galleries;
pub mod gallery_sessions;
pub mod error;

pub type ConnectionPool = Pool<AsyncPgConnection>;

/// A centralized struct for initializing all stores.
pub struct AppStores {
    pub gallery_store: GalleryStore,
    pub gallery_sessions_store: GallerySessionsStore
}

impl AppStores {
    /// Initialize all the stores.
    pub fn new(config: &StoreConfig) -> Self {
        let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(&config.database_url);
        let pool = Pool::builder(config)
            .build()
            .unwrap();

        tracing::info!("Connection pool successfully initialized...");

        Self {
            gallery_store: GalleryStore::new(pool.clone()),
            gallery_sessions_store: GallerySessionsStore::new(pool.clone())
        }
    }
}