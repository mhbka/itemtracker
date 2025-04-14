use chrono::NaiveDateTime;
use uuid::Uuid;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use crate::{domain::{domain_types::UnixUtcDateTime, gallery::{Gallery, GalleryStats}}, models::gallery::{GalleryModel, NewGallery, UpdatedGallery}, schema::{embedded_marketplace_items, gallery_sessions, marketplace_items}};
use super::{error::StoreError, ConnectionPool};

/// For accessing/storing galleries.
#[derive(Clone)]
pub struct GalleryStore {
    pool: ConnectionPool
}

impl GalleryStore {
    /// Initialize the store.
    pub fn new(pool: ConnectionPool) -> Self {
        Self {
            pool
        }
    }

    /// Get the data for a gallery.
    pub async fn get_gallery(&mut self, gallery_id: Uuid) -> Result<Gallery, StoreError> {
        use crate::schema::galleries::dsl::*;
        let mut conn = self.pool.get().await?;
        
        galleries
            .filter(id.eq(gallery_id))
            .first::<GalleryModel>(&mut conn)
            .await
            .map_err(StoreError::from)
    }

    /// Get all galleries under a user.
    pub async fn get_all_galleries(&mut self, uid: Uuid) -> Result<Vec<Gallery>, StoreError> {
        use crate::schema::galleries::dsl::*;
        let mut conn = self.pool.get().await?;
        
        galleries
            .filter(user_id.eq(uid))
            .load::<GalleryModel>(&mut conn)
            .await
            .map_err(StoreError::from)
    }

    /// Add a new gallery.
    pub async fn add_new_gallery(&mut self, new_gallery: NewGallery) -> Result<(), StoreError> {
        use crate::schema::galleries::dsl::*;
        let mut conn = self.pool.get().await?;
        
        diesel::insert_into(galleries)
            .values(&new_gallery)
            .execute(&mut conn)
            .await
            .map(|_| ())
            .map_err(StoreError::from)
    }

    /// Update a gallery's data.
    pub async fn update_gallery(&mut self, gallery_id: Uuid, gallery_changes: UpdatedGallery) -> Result<(), StoreError> {
        use crate::schema::galleries::dsl::*;
        let mut conn = self.pool.get().await?;

        let updated_rows = diesel::update(galleries.filter(id.eq(gallery_id)))
            .set(&gallery_changes)
            .execute(&mut conn)
            .await?;
            
        if updated_rows == 0 {
            return Err(StoreError::NotFound { gallery_id });
        }
        
        Ok(())
    }
    
    /// Delete a gallery.
    pub async fn delete_gallery(&mut self, gallery_id: Uuid) -> Result<(), StoreError> {
        use crate::schema::galleries::dsl::*;
        let mut conn = self.pool.get().await?;
        
        let deleted_rows = diesel::delete(galleries.filter(id.eq(gallery_id)))
            .execute(&mut conn)
            .await?;
            
        if deleted_rows == 0 {
            return Err(StoreError::NotFound { gallery_id });
        }
        
        Ok(())
    }

    /// Get the stats for the gallery.
    pub async fn get_stats(&mut self, gallery_id: Uuid) -> Result<GalleryStats, StoreError> {
        let mut conn = self.pool.get().await?;

        let total_sessions = gallery_sessions::table
            .filter(gallery_sessions::columns::gallery_id.eq(gallery_id))
            .count()
            .get_result::<i64>(&mut conn)
            .await? as u32;
        let total_mercari_items = gallery_sessions::table
            .inner_join(
                embedded_marketplace_items::table.left_join(marketplace_items::table)
            )
            .filter(gallery_sessions::columns::gallery_id.eq(gallery_id))
            .count()
            .get_result::<i64>(&mut conn)
            .await? as u32;
        let latest_scrape = gallery_sessions::table
            .filter(gallery_sessions::columns::gallery_id.eq(gallery_id))
            .order(gallery_sessions::columns::created.desc())
            .select(gallery_sessions::columns::created)
            .first::<NaiveDateTime>(&mut conn)
            .await?;
        Ok(
            GalleryStats { 
                total_sessions, 
                total_mercari_items, 
                latest_scrape: UnixUtcDateTime::new(latest_scrape.and_utc())
            }
        )
    }
}