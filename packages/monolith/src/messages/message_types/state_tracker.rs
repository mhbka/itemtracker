use serde::{Deserialize, Serialize};

/// The types of messages that the state tracker module can take.
#[derive(Debug)]
pub enum StateTrackerMessage {

}

/// The states of a gallery that are being tracked,
/// along with any important ephemeral data.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum GalleryStates {
    SearchScraping { 
        
    },
    ItemScraping {

    },
    ItemAnalysis {

    },
    ImageClassification {

    }
}