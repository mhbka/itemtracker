use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;
use crate::{domain::{eval_criteria::EvaluationCriteria, search_criteria::SearchCriteria}, schema::galleries::{self, mercari_last_scraped_time}};

// Model of the gallery table.
#[derive(Queryable, Identifiable, Debug)]
#[table_name = "galleries"]
pub struct GalleryModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub scraping_periodicity: String,
    pub search_criteria: SearchCriteria,
    pub evaluation_criteria: EvaluationCriteria,
    pub mercari_last_scraped_time: Option<i64>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// For inserting a new gallery.
#[derive(Insertable)]
#[table_name = "galleries"]
pub struct NewGallery {
    pub user_id: Uuid,
    pub scraping_periodicity: String,
    pub search_criteria: SearchCriteria,
    pub evaluation_criteria: EvaluationCriteria,
    pub mercari_last_scraped_time: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// For updating a gallery.
#[derive(AsChangeset)]
#[table_name = "galleries"]
pub struct UpdatedGallery {
    pub scraping_periodicity: Option<String>,
    pub search_criteria: Option<SearchCriteria>,
    pub evaluation_criteria: Option<EvaluationCriteria>,
    pub mercari_last_scraped_time: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl UpdatedGallery {
    /// Get the model for updating marketplace datetimes.
    pub fn update_marketplace_datetimes(mercari: Option<NaiveDateTime>) -> Self {
        Self {
            scraping_periodicity: None,
            search_criteria: None,
            evaluation_criteria: None,
            mercari_last_scraped_time: mercari,
            updated_at: Some(Utc::now().naive_utc())
        }
    }
}