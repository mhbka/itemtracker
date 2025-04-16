use thiserror::Error;
use super::{item_analysis::error::ItemAnalysisError, item_embedder::error::ItemEmbedderError, item_scraper::error::ItemScraperError, search_scraper::error::SearchScraperError, storage::error::StorageError};

/// Errors from each of the pipeline stages.
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
    Storage(#[from] StorageError)
}