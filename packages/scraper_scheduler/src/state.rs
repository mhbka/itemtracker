use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use tokio_cron_scheduler::{JobScheduler, JobSchedulerError};
use uuid::Uuid;
use croner::Cron;
use crate::{config::AppConfig, galleries::Galleries, output::OutputRequester, scheduler::{self, SchedulerWrapper}};

/// "Global" state for the app.
pub struct AppState {
    pub config: AppConfig,
    pub galleries: Arc<Mutex<HashMap<Uuid, Galleries>>>,
    pub scheduler: Arc<Mutex<SchedulerWrapper>>,
    pub output_requester: Arc<Mutex<OutputRequester>>
}

impl AppState {
    /// This initializes the app's state.
    /// TODO: Pull galleries from storage upon startup here.
    pub async fn new() -> Result<Self, JobSchedulerError> {
        let config = AppConfig::init();
        let output_requester = Arc::new(Mutex::new(OutputRequester::new(config.clone())));
        let galleries = Arc::new(Mutex::new(HashMap::new()));
        
        let scheduler = JobScheduler::new().await?;
        scheduler.start().await?;
        let scheduler = Arc::new(Mutex::new(SchedulerWrapper::new(scheduler, output_requester.clone())));
        
        Ok(
            AppState {
            config,
            scheduler,
            galleries,
            output_requester
            }
        )
    }

    /// Add a new gallery.
    /// Returns an error if the gallery already exists, or if the scheduler fails to add it.
    pub async fn add_gallery(&mut self, mut gallery: Galleries) -> Result<(), JobSchedulerError> {
        if let None = self.find_gallery(&gallery.get_gallery_id()).await {
            let mut scheduler = self.scheduler.lock().await;
            let mut galleries = self.galleries.lock().await;
            let gallery_job_id = scheduler.add_gallery(&mut gallery).await?;
            galleries.insert(gallery_job_id, gallery);
            return Ok(());
        }
        Err(JobSchedulerError::CantAdd)
    }

    /// Edit an existing gallery.
    /// Returns an error if the gallery does not exist, or the scheduler fails to add it.
    pub async fn edit_gallery(&mut self, mut gallery: Galleries) -> Result<(), JobSchedulerError> {
        if Cron::new(&gallery.get_gallery_periodicity())
            .parse()
            .is_err() {
                return Err(JobSchedulerError::ParseSchedule);
            }
        if let Some(cur_gallery_job_id) = self.find_gallery(&gallery.get_gallery_id()).await {
            let scheduler = self.scheduler.clone();
            let galleries= self.galleries.clone();
            tokio::spawn(
                async move {
                    let mut scheduler = scheduler.lock().await;
                    let mut galleries = galleries.lock().await;
                    let new_gallery_job_id = scheduler.edit_gallery(cur_gallery_job_id, &mut gallery).await.unwrap(); // TODO: logging?
                    galleries.remove(&cur_gallery_job_id);
                    galleries.insert(new_gallery_job_id, gallery);
                }
            );
            Ok(())
        } else {
            Err(JobSchedulerError::CantAdd)
        }
    }

    /// Get the job Uuid for a gallery.
    pub async fn find_gallery(&mut self, gallery_id: &str) -> Option<Uuid> {
        self.galleries
            .lock()
            .await
            .iter_mut()
            .find(|g| g.1.get_gallery_id() == gallery_id)
            .map(|g| g.0.clone())
    }
}