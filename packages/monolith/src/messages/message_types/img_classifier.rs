use crate::galleries::pipeline_states::GalleryAnalyzedState;

/// The types of messages the image classifer module can take.
#[derive(Debug)]
pub enum ImgClassifierMessage {
    StartClassification { gallery: GalleryAnalyzedState }
}