use message_buses::{MessageSender, MessageReceiver};
use message_types::{
    item_analysis::ItemAnalysisMessage, 
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
/// Handle for the scraper scheduler to receive messages.
pub type ScraperSchedulerReceiver = MessageReceiver<SchedulerMessage>;

/// Handle for sending messages to the scraper.
pub type ScraperSender = MessageSender<ScraperMessage>;
/// Handle for the scraper module to receive messages.
pub type ScraperReceiver = MessageReceiver<ScraperMessage>;

/// Handle for sending the item analysis module messages.
pub type ItemAnalysisSender = MessageSender<ItemAnalysisMessage>;
/// Handle for the item analysis module to receive messages.
pub type ItemAnalysisReceiver = MessageReceiver<ItemAnalysisMessage>;

/// Handle for sending the image classifier module messages.
pub type ImageClassifierSender = MessageSender<ImgClassifierMessage>;
/// Handle for the image classifier module to receive messages.
pub type ImageClassifierReceiver = MessageReceiver<ImgClassifierMessage>;

/// Handle for sending the marketplace items storage module messages.
pub type MarketplaceItemsStorageSender = MessageSender<MarketplaceItemsStorageMessage>;
/// Handle for the marketplace items storage storage module to receive messages.
pub type MarketplaceItemsStorageReceiver = MessageReceiver<MarketplaceItemsStorageMessage>;