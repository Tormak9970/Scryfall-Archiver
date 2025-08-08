use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct BulkDataItem {
  pub name: String,
  pub download_uri: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BulkDataResponse {
  pub data: Vec<BulkDataItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
  pub id: String,
  pub image_uris: Option<ImageUris>,
  pub card_faces: Option<Vec<CardFace>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardFace {
  pub image_uris: Option<ImageUris>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ruling {
  pub object: String,
  pub oracle_id: String,
  pub source: String,
  pub published_at: String,
  pub comment: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ImageUris {
  pub small: Option<String>,
  pub normal: Option<String>,
  pub large: Option<String>,
  pub png: Option<String>,
  pub art_crop: Option<String>,
  pub border_crop: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImagesConfig {
  pub small: bool,
  pub normal: bool,
  pub large: bool,
  pub png: bool,
  pub art_crop: bool,
  pub border_crop: bool,
}