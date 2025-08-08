use log::{info, warn};
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use std::{fs, io};
use std::io::{BufReader, BufWriter, Write};
use std::time::Duration;
use serde_json;
use futures_util::StreamExt;

use std::{env::var, path::{Path, PathBuf}};

use crate::json_stream::iter_json_array;
use crate::types::{BulkDataResponse, Card, ImageUris, ImagesConfig, Ruling};


// const IMAGE_URL_START: &str = "https://cards.scryfall.io/";
const IMAGE_URL_START_LENGTH: usize = 26;

fn url_to_filename(url: &str) -> String {
  let end_index = url.find('?').unwrap_or(0);
  let path_segment = &url[IMAGE_URL_START_LENGTH..end_index];

  return path_segment.to_string();
}

async fn download_card_image(client: &Client, card_id: &str, url: String, images_dir: &Path) -> Result<u8, Box<dyn std::error::Error>> {
  if !url.starts_with("https://cards.scryfall.io/") {
    return Ok(0);
  }
  
  let image_path = url_to_filename(&url);
  let file_path = images_dir.join(&image_path);

  if file_path.exists() {
    info!("Found cache, skipping: {}", file_path.display());
    return Ok(0);
  }

  fs::create_dir_all(file_path.parent().unwrap())?;

  let resp = client.get(&url).send().await?;
    
  let res = fs::write(&file_path, resp.bytes().await?);
  if res.is_err() {
    let err = res.err().unwrap();
    warn!("Failed to write card image for {}: {}", card_id, err.to_string());

    return Ok(2);
  }

  info!("Downloaded: {}", file_path.display());

  return Ok(1);
}

fn bool_var(key: &str) -> bool {
  return (var(key).unwrap_or(String::from("false"))).eq("true");
}

async fn download_card_images(images_config: &ImagesConfig, client: &Client, card_id: &str, image_uris: &ImageUris, images_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
  if images_config.small {
    match image_uris.small.clone() {
      Some(url) => {
        let _ = download_card_image(&client, card_id, url, images_dir).await;
        info!("Downloaded small image for {}", card_id);
      },
      None => {},
    }
  }

  if images_config.normal {
    match image_uris.normal.clone() {
      Some(url) => {
        let _ = download_card_image(&client, card_id, url, images_dir).await;
        info!("Downloaded normal image for {}", card_id);
      },
      None => {},
    }
  }

  if images_config.large {
    match image_uris.large.clone() {
      Some(url) => {
        let _ = download_card_image(&client, card_id, url, images_dir).await;
        info!("Downloaded large image for {}", card_id);
      },
      None => {},
    }
  }

  if images_config.png {
    match image_uris.png.clone() {
      Some(url) => {
        let _ = download_card_image(&client, card_id, url, images_dir).await;
        info!("Downloaded png image for {}", card_id);
      },
      None => {},
    }
  }
  
  if images_config.art_crop {
    match image_uris.art_crop.clone() {
      Some(url) => {
        let _ = download_card_image(&client, card_id, url, images_dir).await;
        info!("Downloaded art_crop image for {}", card_id);
      },
      None => {},
    }
  }
  
  if images_config.border_crop {
    match image_uris.border_crop.clone() {
      Some(url) => {
        let _ = download_card_image(&client, card_id, url, images_dir).await;
        info!("Downloaded border_crop image for {}", card_id);
      },
      None => {},
    }
  }

  return Ok(());
}

async fn fetch_card_images(client: &Client, bulk_data_filename: &PathBuf, images_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
  info!("Downloading card images...");

  let images_config = ImagesConfig {
    small: bool_var("SA_BACKUP_SMALL_IMAGE"),
    normal: bool_var("SA_BACKUP_NORMAL_IMAGE"),
    large: bool_var("SA_BACKUP_LARGE_IMAGE"),
    png: bool_var("SA_BACKUP_PNG_IMAGE"),
    art_crop: bool_var("SA_BACKUP_ART_CROP_IMAGE"),
    border_crop: bool_var("SA_BACKUP_BORDER_CROP_IMAGE"),
  };

  let reader = BufReader::new(std::fs::File::open(bulk_data_filename)?);

  let mut num_downloaded: u64 = 95000;

  for card_res in iter_json_array::<Card, BufReader<std::fs::File>>(reader) {
    if card_res.is_err() {
      let err = card_res.err().unwrap();
      warn!("Failed to read card: {}", err.to_string());
      continue;
    }
    let card = card_res.unwrap();

    if card.image_uris.is_some() {
      let image_uris = card.image_uris.as_ref().unwrap();
      let _ = download_card_images(&images_config, client, &card.id, image_uris, images_dir).await;
    } else {
      let card_faces = card.card_faces.as_ref().expect("card_faces should exist when image_uris are absent");

      for card_face in card_faces {
        if card_face.image_uris.is_some() {
          let image_uris = card_face.image_uris.as_ref().unwrap();
          let _ = download_card_images(&images_config, client, &card.id, image_uris, images_dir).await;
        }
      }
    }
    
    num_downloaded += 1;
    info!("STATUS: {} cards downloaded", num_downloaded);
  }

  return Ok(());
}

async fn download_card_data(client: &Client, bulk_data: BulkDataResponse, images_dir: &PathBuf, output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
  let default_cards_url = bulk_data
      .data
      .into_iter()
      .find(|item| item.name == "Default Cards")
      .expect("Could not find default cards")
      .download_uri;
  
  let bulk_data_filename = output_dir.join("bulk-data.json");

  info!("Default Cards URL: {}", default_cards_url);

  info!("Downloading bulk card data...");

  let data_file_res = std::fs::File::create(&bulk_data_filename);
  if data_file_res.is_err() {
    let err = data_file_res.err().unwrap();
    warn!("Failed to create bulk card data file: {}", err.to_string());

    return Ok(());
  }
  let data_file = data_file_res.ok().unwrap();

  let mut buf_writer = BufWriter::new(data_file);
  
  let mut byte_stream = client
    .get(&default_cards_url)
    .send().await?
    .bytes_stream();

  while let Some(bytes) = byte_stream.next().await {
    let write_res = io::copy(&mut bytes?.as_ref(), &mut buf_writer);

    if write_res.is_err() {
      let err = write_res.err().unwrap();
      warn!("Failed to write bulk card data: {}", err.to_string());

      return Ok(());
    }
  }

  buf_writer.flush()?;
  
  info!("Downloaded bulk card data.");

  let _ = fetch_card_images(&client, &bulk_data_filename, images_dir).await;

  return Ok(());
}

async fn download_card_rulings(client: &Client, bulk_data: BulkDataResponse, output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
  let rulings_url = bulk_data
      .data
      .into_iter()
      .find(|item| item.name == "Rulings")
      .expect("Could not find rulings")
      .download_uri;

  info!("Rulings URL: {}", rulings_url);

  info!("Downloading card ruling data...");
  let rulings_res = client.get(&rulings_url).send().await;
  if rulings_res.is_err() {
    let err = rulings_res.err().unwrap();
    warn!("Failed to fetch ruling data: {}", err.to_string());
    
    return Ok(());
  }

  let rulings: Vec<Ruling> = rulings_res.unwrap().json().await?;
  
  let ruling_data_filename = output_dir.join("ruling-data.json");

  let write_res = fs::write(ruling_data_filename, serde_json::to_string(&rulings)?);

  if write_res.is_err() {
    let err = write_res.err().unwrap();
    warn!("Failed to write ruling data: {}", err.to_string());
  }

  return Ok(());
}

pub async fn archive_scryfall() -> Result<(), Box<dyn std::error::Error>> {
  // Setup
  let output_dir_str = var("SA_DATA_DIR").unwrap_or(String::from("/data"));
  let output_dir  = Path::new(&output_dir_str);
  let images_dir = output_dir.join("card-images");
  
  fs::create_dir_all(&images_dir)?;

  let mut default_headers = HeaderMap::new();
  default_headers.append("User-Agent", HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:140.0) Gecko/20100101 Firefox/140.0"));
  default_headers.append("Accept", HeaderValue::from_static("*/*"));

  let client = Client::builder()
    .default_headers(default_headers)
    .timeout(Duration::from_secs(60))
    .build()?;

  // Download Data
  info!("Fetching Scryfall bulk data list...");
  let bulk_data: BulkDataResponse = client
      .get("https://api.scryfall.com/bulk-data")
      .send().await?
      .json().await?;

  let _ = download_card_data(&client, bulk_data.clone(), &images_dir, output_dir).await;

  let _ = download_card_rulings(&client, bulk_data, output_dir).await;

  info!("Archive Complete.");
  return Ok(());
}
