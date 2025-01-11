use crate::messages::message_types::scraper_scheduler::{DeleteGalleryMessage, EditGalleryMessage, NewGalleryMessage};
use super::ScraperSchedulerModule;

pub(super) async fn handle_new_gallery_msg(msg: NewGalleryMessage, module: &mut ScraperSchedulerModule) {
    let gallery = msg.get_msg();
    let result = module.scheduler.add_gallery(gallery).await;
    if msg.respond(result).is_err() {
        tracing::error!("Was unable to respond to a message for creating a gallery");
    }
}

pub(super) async fn handle_delete_gallery_msg(msg: DeleteGalleryMessage, module: &mut ScraperSchedulerModule) {
    let gallery_id = msg.get_msg().gallery_id;
    let result = module.scheduler.delete_gallery(gallery_id).await;
    if msg.respond(result).is_err() {
        tracing::error!("Was unable to respond to a message for deleting a gallery");
    }
}

pub(super) async fn handle_edit_gallery_msg(msg: EditGalleryMessage, module: &mut ScraperSchedulerModule) {
    let gallery = msg.get_msg();
    let result = module.scheduler.update_gallery(gallery).await;
    if msg.respond(result).is_err() {
        tracing::error!("Was unable to respond to a message for editing a gallery");
    }
}