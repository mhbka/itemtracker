use reqwest::Client;

/// This struct is in charge of scraping items from Mercari.
pub(super) struct MercariItemScraper {
    client: Client
}

impl MercariItemScraper {
    pub fn new() -> Self {
        Self {
            client: Client::new()
        }
    }

    pub fn request(&self, item_ids: Vec<ItemId>) -> Result<String, String> {
        todo!()
    }    
}