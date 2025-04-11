use uuid::Uuid;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use crate::models::galleries::{Gallery, GalleryChanges, NewGallery};
use super::{error::StoreError, ConnectionPool};

/// A cloneable interface for accessing galleries.
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
            .first::<Gallery>(&mut conn)
            .await
            .map_err(StoreError::from)
    }

    /// Get all galleries under a user.
    pub async fn get_all_galleries(&mut self, uid: Uuid) -> Result<Vec<Gallery>, StoreError> {
        use crate::schema::galleries::dsl::*;
        
        let mut conn = self.pool.get().await?;
        
        galleries
            .filter(user_id.eq(uid))
            .load::<Gallery>(&mut conn)
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
    pub async fn update_gallery(&mut self, gallery_id: Uuid, gallery_changes: GalleryChanges) -> Result<(), StoreError> {
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
}