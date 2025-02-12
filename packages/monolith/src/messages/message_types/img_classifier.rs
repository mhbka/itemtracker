use crate::galleries::pipeline_states::GalleryClassifierState;

/// The types of messages the image classifer module can take.
#[derive(Debug)]
pub enum ImgClassifierMessage {
    StartClassification { gallery: GalleryClassifierState }
}