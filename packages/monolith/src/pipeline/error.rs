use thiserror::Error;
use crate::stores::error::StoreError;

use super::{item_analysis::error::ItemAnalysisError, item_embedder::error::ItemEmbedderError, item_scraper::error::ItemScraperError, search_scraper::error::SearchScraperError, storage::error::StorageError};

/// Errors from each of the pipeline stages,
/// or from updating the scheduler after the pipeline has ran.
#[derive(Error, Debug)]
pub enum PipelineError {
    #[error("{0}")]
    SearchScrape(#[from] SearchScraperError),
    #[error("{0}")]
    ItemScrape(#[from] ItemScraperError),
    #[error("{0}")]
    ItemAnalysis(#[from] ItemAnalysisError),
    #[error("{0}")]
    ItemEmbedder(#[from] ItemEmbedderError),
    #[error("{0}")]
    Storage(#[from] StorageError),
    #[error("Failed to update the pipeline: {reason}")]
    FailedToUpdate { reason: String }
}