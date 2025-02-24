use item_embedder::ItemEmbedderModule;
use item_analysis::ItemAnalysisModule;
use item_scraper::ItemScraperModule;
use state_tracker::StateTrackerModule;
use storage::StorageModule;
use tokio::sync::mpsc;
use search_scraper::SearchScraperModule;
use scraper_scheduler::ScraperSchedulerModule;
use tokio::task::JoinHandle;
use crate::{config::AppConfig, messages::{message_buses::MessageSender, ItemAnalysisReceiver, ItemAnalysisSender, ItemEmbedderReceiver, ItemEmbedderSender, ItemScraperReceiver, ItemScraperSender, ScraperSchedulerReceiver, ScraperSchedulerSender, SearchScraperReceiver, SearchScraperSender, StateTrackerReceiver, StateTrackerSender, StorageReceiver, StorageSender}};

pub mod state_tracker;
pub mod scraper_scheduler;
pub mod search_scraper;
pub mod item_scraper;
pub mod item_analysis;
pub mod item_embedder;
pub mod storage;

const MODULE_MESSAGE_BUFFER: usize = 1000;

/// Struct for instantiating the app's modules.
pub struct AppModules {
    state_tracker_module: StateTrackerModule,
    scheduler_module: ScraperSchedulerModule,
    search_scraper_module: SearchScraperModule,
    item_scraper_module: ItemScraperModule,
    analysis_module: ItemAnalysisModule,
    classifier_module: ItemEmbedderModule,
    storage_module: StorageModule
}

impl AppModules {
    /// Initialize the app's modules.
    pub fn init(config: AppConfig, connections: AppModuleConnections) -> Self {
        let state_tracker_module = StateTrackerModule::init(
            config.state_tracker_config, 
            connections.state_tracker.1
        );
        let scheduler_module = ScraperSchedulerModule::init(
            config.scraper_scheduler_config,
            connections.scraper_scheduler.1, 
            connections.search_scraper.0,
            connections.state_tracker.0.clone()
        );
        let search_scraper_module = SearchScraperModule::init(
            config.search_scraper_config, 
            connections.search_scraper.1, 
            connections.state_tracker.0.clone(),
            connections.item_scraper.0,
        );
        let item_scraper_module = ItemScraperModule::init(
            config.item_scraper_config,
            connections.item_scraper.1,
            connections.state_tracker.0.clone(),
            connections.item_analysis.0
        );
        let analysis_module = ItemAnalysisModule::init(
            config.item_analysis_config.clone(),
            connections.item_analysis.1,
            connections.state_tracker.0.clone(),
            connections.image_classifier.0
        );
        let classifier_module = ItemEmbedderModule::init(
            config.img_classifier_config.clone(),
            connections.image_classifier.1,
            connections.state_tracker.0.clone(),
            connections.storage.0.clone()
        );
        let storage_module = StorageModule::init(
            config.storage_config,
            connections.storage.1,
            connections.state_tracker.0.clone()
        );
        AppModules {
            state_tracker_module,
            scheduler_module,
            search_scraper_module,
            item_scraper_module,
            analysis_module,
            classifier_module,
            storage_module
        }
    }

    /// Start running all of the app's modules.
    pub fn run(mut self) -> AppModulesRunningHandles {
        let state_tracker_task = tokio::spawn(async move { self.state_tracker_module.run().await; });
        let scheduler_task = tokio::spawn(async move { self.scheduler_module.run().await; });
        let search_scraper_task = tokio::spawn(async move { self.search_scraper_module.run().await; });
        let item_scraper_task = tokio::spawn(async move { self.item_scraper_module.run().await; });
        let analysis_task = tokio::spawn(async move { self.analysis_module.run().await; });
        let classifier_task = tokio::spawn(async move { self.classifier_module.run().await; });
        let storage_task = tokio::spawn(async move { self.storage_module.run().await; });
        AppModulesRunningHandles {
            state_tracker_task,
            scheduler_task,
            search_scraper_task,
            item_scraper_task,
            analysis_task,
            classifier_task,
            storage_task
        }
    }
}

/// Holds task handles for each module's running tasks.
pub struct AppModulesRunningHandles {
    state_tracker_task: JoinHandle<()>,
    scheduler_task: JoinHandle<()>,
    search_scraper_task: JoinHandle<()>,
    item_scraper_task: JoinHandle<()>,
    analysis_task: JoinHandle<()>,
    classifier_task: JoinHandle<()>,
    storage_task: JoinHandle<()>,
}

/// Struct for initializing inter-module connections.
pub struct AppModuleConnections {
    pub state_tracker: (StateTrackerSender, StateTrackerReceiver),
    pub scraper_scheduler: (ScraperSchedulerSender, ScraperSchedulerReceiver),
    pub search_scraper: (SearchScraperSender, SearchScraperReceiver),
    pub item_scraper: (ItemScraperSender, ItemScraperReceiver),
    pub item_analysis: (ItemAnalysisSender, ItemAnalysisReceiver),
    pub image_classifier: (ItemEmbedderSender, ItemEmbedderReceiver),
    pub storage: (StorageSender, StorageReceiver)
}

impl AppModuleConnections {
    /// Initialize the app module connections.
    pub fn new() -> Self {
        Self {
            state_tracker: Self::init_state_tracker_conn(),
            scraper_scheduler: Self::init_scheduler_conn(),
            search_scraper: Self::init_search_scraper_conn(),
            item_scraper: Self::init_item_scraper_conn(),
            item_analysis: Self::init_item_analysis_conn(),
            image_classifier: Self::init_image_classifier_conn(),
            storage: Self::storage_conn()
        }
    }

    fn init_state_tracker_conn() -> (StateTrackerSender, StateTrackerReceiver) {
        let (sender, receiver) = mpsc::channel(MODULE_MESSAGE_BUFFER);
        let raw_sender = MessageSender::new(sender);
        let sender = StateTrackerSender::new(raw_sender);
        let receiver = StateTrackerReceiver::new(receiver);
        (sender, receiver)
    }

    fn init_scheduler_conn() -> (ScraperSchedulerSender, ScraperSchedulerReceiver) {
        let (sender, receiver) = mpsc::channel(MODULE_MESSAGE_BUFFER);
        let sender = ScraperSchedulerSender::new(sender);
        let receiver = ScraperSchedulerReceiver::new(receiver);
        (sender, receiver)
    }

    fn init_search_scraper_conn() -> (SearchScraperSender, SearchScraperReceiver) {
        let (sender, receiver) = mpsc::channel(MODULE_MESSAGE_BUFFER);
        let sender = SearchScraperSender::new(sender);
        let receiver = SearchScraperReceiver::new(receiver);
        (sender, receiver)
    }

    fn init_item_scraper_conn() -> (ItemScraperSender, ItemScraperReceiver) {
        let (sender, receiver) = mpsc::channel(MODULE_MESSAGE_BUFFER);
        let sender = ItemScraperSender::new(sender);
        let receiver = ItemScraperReceiver::new(receiver);
        (sender, receiver)
    }

    fn init_item_analysis_conn() -> (ItemAnalysisSender, ItemAnalysisReceiver) {
        let (sender, receiver) = mpsc::channel(MODULE_MESSAGE_BUFFER);
        let sender = ItemAnalysisSender::new(sender);
        let receiver = ItemAnalysisReceiver::new(receiver);
        (sender, receiver)
    }

    fn init_image_classifier_conn() -> (ItemEmbedderSender, ItemEmbedderReceiver) {
        let (sender, receiver) = mpsc::channel(MODULE_MESSAGE_BUFFER);
        let sender = ItemEmbedderSender::new(sender);
        let receiver = ItemEmbedderReceiver::new(receiver);
        (sender, receiver)
    }

    fn storage_conn() -> (StorageSender, StorageReceiver) {
        let (sender, receiver) = mpsc::channel(MODULE_MESSAGE_BUFFER);
        let sender = StorageSender::new(sender);
        let receiver = StorageReceiver::new(receiver);
        (sender, receiver)
    }
}