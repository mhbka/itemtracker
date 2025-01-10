use image_analysis::ImageAnalysisModule;
use tokio::sync::mpsc;

use scraper::ScraperModule;
use scraper_scheduler::ScraperSchedulerModule;
use tokio::task::JoinHandle;
use crate::{config::AppConfig, messages::{ImgAnalysisReceiver, ImgAnalysisSender, MarketplaceItemsStorageReceiver, MarketplaceItemsStorageSender, ScraperReceiver, ScraperSchedulerReceiver, ScraperSchedulerSender, ScraperSender}};

pub mod web_backend;
pub mod scraper_scheduler;
pub mod scraper;
pub mod image_analysis;
pub mod image_classifier;
pub mod storage;

const MODULE_MESSAGE_BUFFER: usize = 1000;

/// Struct for instantiating the app's modules.
pub struct AppModules {
    scheduler_module: ScraperSchedulerModule,
    scraper_module: ScraperModule,
    analysis_module: ImageAnalysisModule
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
            connections.marketplace_items_storage.0, 
            connections.img_analysis.0
        );
        let analysis_module = ImageAnalysisModule::init(
            config.img_analysis_config.clone(),
            connections.img_analysis.1
        );
        AppModules {
            scheduler_module,
            scraper_module,
            analysis_module
        }
    }

    /// Start running all of the app's modules.
    pub fn run(mut self) -> AppModulesRunningHandles {
        let scheduler_task = tokio::spawn(async move { self.scheduler_module.run().await; });
        let scraper_task = tokio::spawn(async move { self.scraper_module.run().await; });
        let analysis_task = tokio::spawn(async move { self.analysis_module.run().await; });
        AppModulesRunningHandles {
            scheduler_task,
            scraper_task,
            analysis_task
        }
    }
}

/// Holds task handles for each module's running tasks.
pub struct AppModulesRunningHandles {
    scheduler_task: JoinHandle<()>,
    scraper_task: JoinHandle<()>,
    analysis_task: JoinHandle<()>
}

/// Struct for initializing inter-module connections.
pub struct AppModuleConnections {
    pub scraper_scheduler: (ScraperSchedulerSender, ScraperSchedulerReceiver),
    pub scraper: (ScraperSender, ScraperReceiver),
    pub img_analysis: (ImgAnalysisSender, ImgAnalysisReceiver),
    pub marketplace_items_storage: (MarketplaceItemsStorageSender, MarketplaceItemsStorageReceiver)
}

impl AppModuleConnections {
    /// Initialize the app module connections.
    pub fn new() -> Self {
        Self {
            scraper_scheduler: Self::init_scheduler_conn(),
            scraper: Self::init_scraper_conn(),
            img_analysis: Self::init_img_analysis_conn(),
            marketplace_items_storage: Self::init_marketplace_items_storage_conn()
        }
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

    fn init_img_analysis_conn() -> (ImgAnalysisSender, ImgAnalysisReceiver) {
        let (sender, receiver) = mpsc::channel(MODULE_MESSAGE_BUFFER);
        let sender = ImgAnalysisSender::new(sender);
        let receiver = ImgAnalysisReceiver::new(receiver);
        (sender, receiver)
    }

    fn init_marketplace_items_storage_conn() -> (MarketplaceItemsStorageSender, MarketplaceItemsStorageReceiver) {
        let (sender, receiver) = mpsc::channel(MODULE_MESSAGE_BUFFER);
        let sender = MarketplaceItemsStorageSender::new(sender);
        let receiver = MarketplaceItemsStorageReceiver::new(receiver);
        (sender, receiver)
    }
}