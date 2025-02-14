use message_buses::{MessageSender, MessageReceiver};
use message_types::{
    img_classifier::ImgClassifierMessage, item_analysis::ItemAnalysisMessage, item_scraper::ItemScraperMessage, scraper_scheduler::SchedulerMessage, search_scraper::SearchScraperMessage, state_tracker::StateTrackerMessage, storage::marketplace_items::MarketplaceItemsStorageMessage, web_backend::WebBackendMessage
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

/// Handle for sending messages to the search scraper.
pub type SearchScraperSender = MessageSender<SearchScraperMessage>;
/// Handle for the search scraper module to receive messages.
pub type SearchScraperReceiver = MessageReceiver<SearchScraperMessage>;

/// Handle for sending messages to the item scraper.
pub type ItemScraperSender = MessageSender<ItemScraperMessage>;
/// Handle for the item scraper module to receive messages.
pub type ItemScraperReceiver = MessageReceiver<ItemScraperMessage>;

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

/// Handle for sending the scraper scheduler messages.
pub type StateTrackerSender = MessageSender<StateTrackerMessage>;
/// Handle for the scraper scheduler to receive messages.
pub type StateTrackerReceiver = MessageReceiver<StateTrackerMessage>;