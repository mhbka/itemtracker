use uuid::Uuid;
use crate::{domain::eval_criteria::EvaluationCriteria, schema::*};
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, Identifiable, Associations};

/// Model of the `gallery_sessions` table.
#[derive(Queryable, Identifiable, Debug, Clone)]
#[table_name = "gallery_sessions"]
pub struct GallerySessionModel {
    pub id: u32,
    pub gallery_id: Uuid,
    pub created: NaiveDateTime,
    pub used_evaluation_criteria: EvaluationCriteria
}

/// For inserting a new gallery session.
#[derive(Insertable, Debug, Clone)]
#[table_name = "gallery_sessions"]
pub struct NewGallerySession {
    pub gallery_id: Uuid,
    pub created: NaiveDateTime,
    pub used_evaluation_criteria: EvaluationCriteria
}