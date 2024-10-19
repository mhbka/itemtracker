use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Galleries {
    Mercari(MercariGallery)
}

impl Galleries {
    pub fn get_gallery_id(&self) -> String {
        match self {
            Galleries::Mercari(mercari_gallery) => mercari_gallery.gallery_id.clone(),
        }
    } 

    pub fn get_gallery_periodicity(&self) -> String {
        match self {
            Galleries::Mercari(mercari_gallery) => mercari_gallery.scraping_periodicity.clone(),
        }
    }   
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MercariGallery {
    pub gallery_id: String,
    pub search_criteria: MercariSearchCriteria,
    pub scraping_periodicity: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MercariSearchCriteria {
    pub keyword: Option<String>,
    pub excludeKeyword: Option<String>
}