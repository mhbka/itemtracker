use crate::{domain::{domain_types::Marketplace, gallery_session::{GallerySession, SessionId}, pipeline_states::GalleryFinalState}, models::{embedded_item::{EmbeddedItemModel, NewEmbeddedMarketplaceItem}, gallery::UpdatedGallery, gallery_session::{GallerySessionModel, NewGallerySession}, item::NewItem}, schema::{embedded_marketplace_items, galleries, gallery_sessions, marketplace_items}};
use super::{error::StoreError, ConnectionPool};
use chrono::Utc;
use diesel::{associations::HasTable, dsl::update, insert_into, prelude::*};
use diesel_async::{AsyncConnection, RunQueryDsl};
use scoped_futures::ScopedFutureExt;

/// For accessing/storing gallery sessions.
pub struct GallerySessionsStore {
    pool: ConnectionPool
}

impl GallerySessionsStore {
    /// Initialize the store.
    pub fn new(pool: ConnectionPool) -> Self {
        Self {
            pool
        }
    }

    /// Add a new gallery session.
    pub async fn add_new_session(&mut self, state: GalleryFinalState) -> Result<SessionId, StoreError> {
        let mut conn = self.pool.get().await?;

        // TODO: move pieces of this into the models themselves
        conn.transaction::<_, StoreError, _>(|conn| async move {
                // create the new session
                let new_session = NewGallerySession {
                    gallery_id: *state.gallery_id.clone(),
                    created: Utc::now().naive_utc(),
                    used_evaluation_criteria: state.used_evaluation_criteria
                };
                let new_session_id: i32 = insert_into(gallery_sessions::table)
                    .values(&new_session)
                    .returning(gallery_sessions::columns::id)
                    .get_result(conn)
                    .await?;

                // insert the embedded items' data and get back their IDs
                let embedded_items_data: Vec<_> = state.items
                    .iter()
                    .flat_map(|(marketplace, items)| {
                        match marketplace {
                            Marketplace::Mercari => {
                                items.embedded_items
                                    .iter()
                                    .map(|e| NewItem::convert(marketplace.clone(), &e.item))
                                    .collect::<Vec<_>>()
                            }
                        }
                    })
                    .collect();
                let embedded_items_data_ids: Vec<i32> = insert_into(marketplace_items::table)
                    .values(&embedded_items_data)
                    .returning(marketplace_items::columns::id)
                    .get_results(conn)
                    .await?;
                
                // insert the embedded items themselves using the data IDs
                let embedded_items: Vec<_> = state.items
                    .iter()
                    .flat_map(|(marketplace, items)| {
                        match marketplace {
                            Marketplace::Mercari => {
                                items.embedded_items
                                    .iter()
                                    .zip(embedded_items_data_ids.iter())
                                    .map(|(item, item_id)| NewEmbeddedMarketplaceItem::convert(*item_id, new_session_id, item))
                                    .collect::<Vec<_>>()
                            }
                        }
                    })
                    .collect();
                insert_into(embedded_marketplace_items::table)
                    .values(&embedded_items)
                    .execute(conn)
                    .await?;

                // update the marketplace datetimes in the gallery
                // NOTE: if another marketplace is added, we need to add them all into 1 update instead of doing once for Mercari only.
                for (marketplace, updated_datetime) in state.marketplace_updated_datetimes {
                    match marketplace {
                        Marketplace::Mercari => {
                            let gallery_update = UpdatedGallery::update_marketplace_datetimes(Some(updated_datetime.naive_utc()));
                            update(galleries::table)
                                .set(&gallery_update)
                                .execute(conn);
                        }
                    }
                }

                Ok(new_session_id)
            }.scope_boxed())
            .await
    }
    
    /// Get a gallery session.
    pub async fn get_session(&mut self, id: SessionId) -> Result<GallerySession, StoreError> {
        todo!()
    }
}