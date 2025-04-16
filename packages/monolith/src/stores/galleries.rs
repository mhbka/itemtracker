use chrono::NaiveDateTime;
use uuid::Uuid;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use crate::{domain::{domain_types::UnixUtcDateTime, gallery::{Gallery, GalleryStats}, pipeline_states::GallerySchedulerState}, models::gallery::{GalleryModel, NewGallery, UpdatedGallery}, schema::{embedded_marketplace_items, galleries, gallery_sessions, marketplace_items}};
use super::{error::{StoreError, StoreResult}, ConnectionPool};

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

    /// Return if the gallery belongs to a user.
    pub async fn gallery_belongs_to_user(&mut self, gallery_id: Uuid, uid: Uuid) -> StoreResult<bool> {
        use crate::schema::galleries::dsl::*;
        let mut conn = self.pool.get().await?;

        let count = galleries
            .filter(id.eq(gallery_id))
            .filter(user_id.eq(uid))
            .count()
            .get_result::<i64>(&mut conn)
            .await?;
        
        Ok(count > 0)
    }

    /// Get the data for a gallery.
    pub async fn get_gallery(&mut self, gallery_id: Uuid) -> StoreResult<Gallery> {
        use crate::schema::galleries::dsl::*;
        let mut conn = self.pool.get().await?;
        
        galleries
            .filter(id.eq(gallery_id))
            .first::<GalleryModel>(&mut conn)
            .await
            .map_err(StoreError::from)
    }

    /// Get all galleries under a user.
    pub async fn get_all_galleries(&mut self, uid: Uuid) -> StoreResult<Vec<Gallery>> {
        use crate::schema::galleries::dsl::*;
        let mut conn = self.pool.get().await?;
        
        galleries
            .filter(user_id.eq(uid))
            .load::<GalleryModel>(&mut conn)
            .await
            .map_err(StoreError::from)
    }

    /// Add a new gallery and returns it.
    pub async fn add_new_gallery(&mut self, new_gallery: NewGallery) -> StoreResult<Gallery> {
        use crate::schema::galleries::dsl::*;
        let mut conn = self.pool.get().await?;
        
        diesel::insert_into(galleries)
            .values(&new_gallery)
            .returning(GalleryModel::as_select())
            .get_result(&mut conn)
            .await
            .map_err(StoreError::from)
    }

    /// Update a gallery's data, returning the updated gallery.
    pub async fn update_gallery(&mut self, gallery_id: Uuid, gallery_changes: UpdatedGallery) -> StoreResult<Gallery> {
        use crate::schema::galleries::dsl::*;
        let mut conn = self.pool.get().await?;

        let updated_gallery = diesel::update(galleries.filter(id.eq(gallery_id)))
            .set(&gallery_changes)
            .returning(GalleryModel::as_select())
            .get_result(&mut conn)
            .await?;
        
        Ok(updated_gallery)
    }
    
    /// Delete a gallery.
    pub async fn delete_gallery(&mut self, gallery_id: Uuid) -> StoreResult<()> {
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
    pub async fn get_stats(&mut self, gallery_id: Uuid) -> StoreResult<GalleryStats> {
        let mut conn = self.pool.get().await?;

        let name = galleries::table
            .filter(galleries::columns::id.eq(gallery_id))
            .select(galleries::columns::name)
            .get_result::<String>(&mut conn)
            .await?;
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
            .await
            .optional()?
            .map(|dt| UnixUtcDateTime::new(dt.and_utc()));
        Ok(
            GalleryStats { 
                name,
                total_sessions, 
                total_mercari_items, 
                latest_scrape
            }
        )
    }

    /// Get stats for all galleries under a user.
    pub async fn get_all_gallery_stats(&mut self, uid: Uuid) -> StoreResult<Vec<(Uuid, GalleryStats)>> {
        use crate::schema::galleries::dsl::*;
        let mut conn = self.pool.get().await?;

        let gallery_ids = galleries
            .filter(user_id.eq(uid))
            .select(id)
            .load::<Uuid>(&mut conn)
            .await?;

        let mut results = Vec::new();
        for gallery_id in gallery_ids {
            let stats = self.get_stats(gallery_id).await?;
            results.push((gallery_id, stats));
        }

        Ok(results)
    }

    /// Get the initial state for all active galleries.
    /// 
    /// ### NOTE
    /// This should be sparingly called (ideally only once during app initialization), since it pulls every single gallery row.
    pub async fn initial_gallery_tasks(&mut self) -> StoreResult<Vec<GallerySchedulerState>> {
        use crate::schema::galleries::dsl::*;
        let mut conn = self.pool.get().await?;

        let all_galleries = galleries
            .filter(is_active.eq(true))
            .get_results::<GalleryModel>(&mut conn)
            .await?;
        let states = all_galleries
            .into_iter()
            .map(|g| g.to_scheduler_state())
            .collect();

        Ok(states)
    }
}