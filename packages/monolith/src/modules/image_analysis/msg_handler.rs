//! This module holds handlers for messages received by the module.
//! 
//! The only reason for putting these here is to make the module file itself neater.
use crate::messages::message_types::img_analysis::StartAnalysisJobMessage;
use super::ImageAnalysisModule;

pub(super) async fn handle_start_analysis_msg(msg: StartAnalysisJobMessage, module: &mut ImageAnalysisModule) {
    let num_items: Vec<_> = msg.get_msg().gallery.items.marketplace_items.values().map(|v| v.len()).collect();
    tracing::info!("WE GOT {num_items:?} ITEMS");
}