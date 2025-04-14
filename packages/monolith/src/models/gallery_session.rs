use uuid::Uuid;
use crate::{domain::{domain_types::UnixUtcDateTime, eval_criteria::EvaluationCriteria, gallery_session::GallerySession, pipeline_items::EmbeddedMarketplaceItem}, schema::*};
use chrono::NaiveDateTime;
use diesel::{pg::Pg, Associations, Identifiable, Insertable, Queryable, Selectable};

/// Model of the `gallery_sessions` table.
#[derive(Queryable, Selectable, Identifiable, Debug, Clone)]
#[diesel(check_for_backend(Pg))]
#[table_name = "gallery_sessions"]
pub struct GallerySessionModel {
    pub id: i32,
    pub gallery_id: Uuid,
    pub created: NaiveDateTime,
    pub used_evaluation_criteria: EvaluationCriteria
}

impl GallerySessionModel {
    /// Convert to the domain type.
    pub fn convert_to(self, mercari_items: Vec<EmbeddedMarketplaceItem>) -> GallerySession {
        GallerySession {
            id: self.id,
            gallery_id: self.gallery_id.into(),
            created: UnixUtcDateTime::new(self.created.and_utc()),
            used_evaluation_criteria: self.used_evaluation_criteria,
            mercari_items
        }
    }
}

/// For inserting a new gallery session.
#[derive(Insertable, Debug, Clone)]
#[table_name = "gallery_sessions"]
pub struct NewGallerySession {
    pub gallery_id: Uuid,
    pub created: NaiveDateTime,
    pub used_evaluation_criteria: EvaluationCriteria
}