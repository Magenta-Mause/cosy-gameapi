use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Asset {
    pub width: u32,
    pub height: u32,
    pub url: String,
}

impl From<steamgriddb_api::images::Image> for Asset {
    fn from(image: steamgriddb_api::images::Image) -> Self {
        Asset {
            width: image.width,
            height: image.height,
            url: image.url,
        }
    }
}

#[derive(Serialize, Clone)]
pub struct AssetList {
    pub assets: Vec<Asset>,
    pub is_final: bool,
}
