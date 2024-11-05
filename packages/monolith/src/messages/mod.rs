use message_buses::{MessageSender, MessageReceiver};
use message_types::{
    img_analysis::ImgAnalysisMessage, 
    img_classifier::ImgClassifierMessage, 
    scraper_scheduler::SchedulerMessage, 
    scraper::ScraperMessage,
    web_backend::WebBackendMessage,
    storage::marketplace_items::MarketplaceItemsStorageMessage
};

pub mod message_buses;
pub mod message_types;

/// Handle for sending the web backend messages.
pub type WebBackendSender = MessageSender<WebBackendMessage>;
/// Handle for the web backend to receive messages.
pub type WebBackendReceiver = MessageReceiver<WebBackendMessage>;

/// Handle for sending the scraper scheduler messages.
pub type ScraperSchedulerSender = MessageSender<SchedulerMessage>;
/// Handle fo the scraper scheduler to receive messages.
pub type ScraperSchedulerReceiver = MessageReceiver<SchedulerMessage>;

/// Handle for sending messages to the scraper.
pub type ScraperSender = MessageSender<ScraperMessage>;
/// Handle for the scraper module to receive messages.
pub type ScraperReceiver = MessageReceiver<ScraperMessage>;

/// Handle for sending the image analysis module messages.
pub type ImgAnalysisSender = MessageSender<ImgAnalysisMessage>;
/// Handle for the image analysis module to receive messages.
pub type ImgAnalysisReceiver = MessageReceiver<ImgAnalysisMessage>;

/// Handle for sending the image classifier module messages.
pub type ImgClassifierSender = MessageSender<ImgClassifierMessage>;
/// Handle for the image classifier module to receive messages.
pub type ImgClassifierReceiver = MessageReceiver<ImgClassifierMessage>;

/// Handle for sending the marketplace items storage module messages.
pub type MarketplaceItemsStorageSender = MessageSender<MarketplaceItemsStorageMessage>;
/// Handle for the marketplace items storage storage module to receive messages.
pub type MarketplaceItemsStorageReceiver = MessageReceiver<MarketplaceItemsStorageMessage>;