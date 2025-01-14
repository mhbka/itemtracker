//! This module holds handlers for messages received by the module.
//! 
//! The only reason for putting these here is to make the module file itself neater.
use crate::messages::message_types::item_analysis::StartAnalysisJob;
use super::ItemAnalysisModule;

// TODO: 
pub(super) async fn handle_start_analysis_msg(msg: StartAnalysisJob, module: &mut ItemAnalysisModule) {
    tracing::trace!("Received message to begin analysis for gallery {}", msg.gallery.gallery_id);
    let gallery = msg.gallery;
    let gallery_id = gallery.gallery_id.clone();
    if module.galleries_in_progress.contains(&gallery_id) {
        tracing::error!("Gallery {} is already being processed by the item analysis module", gallery.gallery_id);
        return;
    }
    module.galleries_in_progress.push(gallery_id.clone());
    match module.llm_requester
        .handle_gallery(gallery)
        .await {
            Ok(_) => {
                
            },
            Err(err) => {

            }
    }
    module.galleries_in_progress.retain(|g| g != &gallery_id);
}