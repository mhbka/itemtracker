
use crate::{domain::{domain_types::{Marketplace, UnixUtcDateTime}, gallery_session::{GallerySession, GallerySessionStats, SessionId}, pipeline_states::GalleryFinalState}, models::{embedded_item::{EmbeddedItemModel, EmbeddedItemWithoutEmbeddings, NewEmbeddedMarketplaceItem}, gallery::UpdatedGallery, gallery_session::{GallerySessionModel, NewGallerySession}, item::{ItemModel, NewItem}}, schema::{embedded_marketplace_items, galleries, gallery_sessions, marketplace_items}};
use super::{error::{StoreError, StoreResult}, ConnectionPool};
use chrono::Utc;
use diesel::{dsl::{count_star, update}, insert_into, prelude::*, upsert::excluded};
use diesel_async::{AsyncConnection, RunQueryDsl};
use scoped_futures::ScopedFutureExt;
use uuid::Uuid;

/// For accessing/storing gallery sessions.
#[derive(Clone)]
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

    /// Return if the session's gallery belongs to a user.
    pub async fn session_belongs_to_user(&mut self, session_id: i32, uid: Uuid) -> StoreResult<bool> {
        let mut conn = self.pool.get().await?;

        let count = gallery_sessions::table
            .inner_join(galleries::table)
            .filter(gallery_sessions::columns::id.eq(session_id))
            .filter(galleries::columns::user_id.eq(uid))
            .count()
            .get_result::<i64>(&mut conn)
            .await?;
        
        Ok(count > 0)
    }

    /// Store a new gallery session.
    pub async fn add_new_session(&mut self, state: GalleryFinalState) -> StoreResult<SessionId> {
        let mut conn = self.pool.get().await?;

        // TODO: move pieces of this into the models themselves
        conn.transaction::<_, StoreError, _>(|conn| async move {
                // create and insert the new session
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
                        // below are for conflicts; ie, a marketplace item being scraped and inserted again
                        .on_conflict((marketplace_items::marketplace, marketplace_items::item_id))
                        .do_update()
                        .set(( 
                            // for brevity, I ommitted seller_id, category, item_condition
                            marketplace_items::columns::name.eq(excluded(marketplace_items::columns::name)),
                            marketplace_items::columns::price.eq(excluded(marketplace_items::columns::price)),
                            marketplace_items::columns::description.eq(excluded(marketplace_items::columns::description)),
                            marketplace_items::columns::status.eq(excluded(marketplace_items::columns::status)),
                            marketplace_items::columns::thumbnails.eq(excluded(marketplace_items::columns::thumbnails)),
                            marketplace_items::columns::updated.eq(excluded(marketplace_items::columns::updated)),
                        ))
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
                                .execute(conn)
                                .await?;
                        }
                    }
                }

                Ok(new_session_id)
            }.scope_boxed())
            .await
    }
    
    /// Get a gallery session.
    pub async fn get_session(&mut self, id: SessionId) -> StoreResult<GallerySession> {
        let mut conn = self.pool.get().await?;

        let session_model = gallery_sessions::table
            .filter(gallery_sessions::columns::id.eq(id))
            .first::<GallerySessionModel>(&mut conn)
            .await?;

        let items = embedded_marketplace_items::table
            .inner_join(marketplace_items::table)
            .filter(embedded_marketplace_items::gallery_session_id.eq(session_model.id))
            .select(<(EmbeddedItemWithoutEmbeddings, ItemModel)>::as_select())
            .load::<(EmbeddedItemWithoutEmbeddings, ItemModel)>(&mut conn)
            .await?;
        let items = items
            .into_iter()
            .map(|(embedded_item, item)| {
                let item = item.convert_to();
                let embedded_item = embedded_item.convert_to(item);
                embedded_item
            })
            .collect();

        let session = session_model.convert_to(items);
        Ok(session)
    }

    /// Get a gallery session's stats.
    pub async fn get_session_stats(&mut self, id: SessionId) -> StoreResult<GallerySessionStats> {
        let mut conn = self.pool.get().await?;

        let session_model = gallery_sessions::table
            .filter(gallery_sessions::columns::id.eq(id))
            .first::<GallerySessionModel>(&mut conn)
            .await?;
        let total_items = embedded_marketplace_items::table
            .filter(embedded_marketplace_items::columns::gallery_session_id.eq(id))
            .count()
            .get_result::<i64>(&mut conn)
            .await? as u32;

        let stats = GallerySessionStats {
            created: UnixUtcDateTime::new(session_model.created.and_utc()),
            used_evaluation_criteria: session_model.used_evaluation_criteria,
            total_items
        };
        Ok(stats)
    }

    /// Get the stats of all sessions under a gallery.
    pub async fn get_all_session_stats(&mut self, gallery_id: Uuid) -> StoreResult<Vec<(SessionId, GallerySessionStats)>> {
        let mut conn = self.pool.get().await?;

        let session_models = gallery_sessions::table
            .filter(gallery_sessions::columns::gallery_id.eq(gallery_id))
            .order(gallery_sessions::columns::created.desc())
            .get_results::<GallerySessionModel>(&mut conn)
            .await?;
        let session_ids: Vec<_> = session_models.iter().map(|s| s.id).collect();

        // TODO: verify this is correct
        let counts = embedded_marketplace_items::table
            .filter(embedded_marketplace_items::columns::gallery_session_id.eq_any(&session_ids))
            .group_by(embedded_marketplace_items::columns::gallery_session_id)
            .select((
                embedded_marketplace_items::columns::gallery_session_id,
                count_star()
            ))
            .load::<(SessionId, i64)>(&mut conn)
            .await?;

        let stats = session_models
            .into_iter()
            .map(|session| {
                let count = match counts.iter().find(|(id, _)| *id == session.id) {
                    Some((_, count)) => *count,
                    None => 0
                };
                let stats = GallerySessionStats {
                    created: UnixUtcDateTime::new(session.created.and_utc()),
                    used_evaluation_criteria: session.used_evaluation_criteria,
                    total_items: count as u32
                };
                (session.id, stats)
            })
            .collect();
        
        Ok(stats)
    }
}