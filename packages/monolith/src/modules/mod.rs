use image_classifier::ImageClassifierModule;
use item_analysis::ItemAnalysisModule;
use tokio::sync::mpsc;
use scraper::ScraperModule;
use scraper_scheduler::ScraperSchedulerModule;
use tokio::task::JoinHandle;
use crate::{config::AppConfig, messages::{ImageClassifierReceiver, ImageClassifierSender, ItemAnalysisReceiver, ItemAnalysisSender, MarketplaceItemsStorageReceiver, MarketplaceItemsStorageSender, ScraperReceiver, ScraperSchedulerReceiver, ScraperSchedulerSender, ScraperSender, StateTrackerReceiver, StateTrackerSender}};

pub mod web_backend;
pub mod state_tracker;
pub mod scraper_scheduler;
pub mod scraper;
pub mod item_analysis;
pub mod image_classifier;
pub mod storage;

const MODULE_MESSAGE_BUFFER: usize = 1000;

/// Struct for instantiating the app's modules.
pub struct AppModules {
    scheduler_module: ScraperSchedulerModule,
    scraper_module: ScraperModule,
    analysis_module: ItemAnalysisModule,
    classifier_module: ImageClassifierModule
}

impl AppModules {
    /// Initialize the app's modules.
    pub fn init(config: &AppConfig, connections: AppModuleConnections) -> Self {
        let scheduler_module = ScraperSchedulerModule::init(
            config.scraper_scheduler_config.clone(),
            connections.scraper_scheduler.1, 
            connections.scraper.0
        );
        let scraper_module = ScraperModule::init(
            config.scraper_config.clone(), 
            connections.scraper.1, 
            connections.state_tracker.0,
            connections.marketplace_items_storage.0, 
            connections.item_analysis.0
        );
        let analysis_module = ItemAnalysisModule::init(
            config.item_analysis_config.clone(),
            connections.item_analysis.1,
            connections.image_classifier.0
        );
        let classifier_module = ImageClassifierModule::init(
            config.img_classifier_config.clone(),
            connections.image_classifier.1
        );
        AppModules {
            scheduler_module,
            scraper_module,
            analysis_module,
            classifier_module
        }
    }

    /// Start running all of the app's modules.
    pub fn run(mut self) -> AppModulesRunningHandles {
        let scheduler_task = tokio::spawn(async move { self.scheduler_module.run().await; });
        let scraper_task = tokio::spawn(async move { self.scraper_module.run().await; });
        let analysis_task = tokio::spawn(async move { self.analysis_module.run().await; });
        let classifier_task = tokio::spawn(async move { self.classifier_module.run().await; });
        AppModulesRunningHandles {
            scheduler_task,
            scraper_task,
            analysis_task,
            classifier_task
        }
    }
}

/// Holds task handles for each module's running tasks.
pub struct AppModulesRunningHandles {
    scheduler_task: JoinHandle<()>,
    scraper_task: JoinHandle<()>,
    analysis_task: JoinHandle<()>,
    classifier_task: JoinHandle<()>,
}

/// Struct for initializing inter-module connections.
pub struct AppModuleConnections {
    pub state_tracker: (StateTrackerSender, StateTrackerReceiver),
    pub scraper_scheduler: (ScraperSchedulerSender, ScraperSchedulerReceiver),
    pub scraper: (ScraperSender, ScraperReceiver),
    pub item_analysis: (ItemAnalysisSender, ItemAnalysisReceiver),
    pub image_classifier: (ImageClassifierSender, ImageClassifierReceiver),
    pub marketplace_items_storage: (MarketplaceItemsStorageSender, MarketplaceItemsStorageReceiver)
}

impl AppModuleConnections {
    /// Initialize the app module connections.
    pub fn new() -> Self {
        Self {
            state_tracker: Self::init_state_tracker_conn(),
            scraper_scheduler: Self::init_scheduler_conn(),
            scraper: Self::init_scraper_conn(),
            item_analysis: Self::init_item_analysis_conn(),
            image_classifier: Self::init_image_classifier_conn(),
            marketplace_items_storage: Self::init_marketplace_items_storage_conn()
        }
    }

    fn init_state_tracker_conn() -> (StateTrackerSender, StateTrackerReceiver) {
        let (sender, receiver) = mpsc::channel(MODULE_MESSAGE_BUFFER);
        let sender = StateTrackerSender::new(sender);
        let receiver = StateTrackerReceiver::new(receiver);
        (sender, receiver)
    }

    fn init_scheduler_conn() -> (ScraperSchedulerSender, ScraperSchedulerReceiver) {
        let (sender, receiver) = mpsc::channel(MODULE_MESSAGE_BUFFER);
        let sender = ScraperSchedulerSender::new(sender);
        let receiver = ScraperSchedulerReceiver::new(receiver);
        (sender, receiver)
    }

    fn init_scraper_conn() -> (ScraperSender, ScraperReceiver) {
        let (sender, receiver) = mpsc::channel(MODULE_MESSAGE_BUFFER);
        let sender = ScraperSender::new(sender);
        let receiver = ScraperReceiver::new(receiver);
        (sender, receiver)
    }

    fn init_item_analysis_conn() -> (ItemAnalysisSender, ItemAnalysisReceiver) {
        let (sender, receiver) = mpsc::channel(MODULE_MESSAGE_BUFFER);
        let sender = ItemAnalysisSender::new(sender);
        let receiver = ItemAnalysisReceiver::new(receiver);
        (sender, receiver)
    }

    fn init_image_classifier_conn() -> (ImageClassifierSender, ImageClassifierReceiver) {
        let (sender, receiver) = mpsc::channel(MODULE_MESSAGE_BUFFER);
        let sender = ImageClassifierSender::new(sender);
        let receiver = ImageClassifierReceiver::new(receiver);
        (sender, receiver)
    }

    fn init_marketplace_items_storage_conn() -> (MarketplaceItemsStorageSender, MarketplaceItemsStorageReceiver) {
        let (sender, receiver) = mpsc::channel(MODULE_MESSAGE_BUFFER);
        let sender = MarketplaceItemsStorageSender::new(sender);
        let receiver = MarketplaceItemsStorageReceiver::new(receiver);
        (sender, receiver)
    }
}