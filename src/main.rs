use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::fs;
use std::time::Duration;
use serde_json;

use std::{env::var, path::{Path, PathBuf}};
use dotenv::dotenv;

#[derive(Debug, Deserialize, Clone)]
struct BulkDataItem {
  name: String,
  download_uri: String,
}

#[derive(Debug, Deserialize, Clone)]
struct BulkDataResponse {
  data: Vec<BulkDataItem>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Card {
  id: String,
  image_uris: Option<ImageUris>,
  card_faces: Option<Vec<CardFace>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CardFace {
  image_uris: Option<ImageUris>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Ruling {
  object: String,
  oracle_id: String,
  source: String,
  published_at: String,
  comment: String,
}


#[derive(Debug, Serialize, Deserialize)]
struct ImageUris {
  // small: Option<String>,
  // normal: Option<String>,
  // large: Option<String>,
  png: Option<String>,
  // art_crop: Option<String>,
  // border_crop: Option<String>,
}

// const IMAGE_URL_START: &str = "https://cards.scryfall.io/";
const IMAGE_URL_START_LENGTH: usize = 26;

fn url_to_filename(url: &str) -> String {
  let end_index = url.find('?').unwrap_or(0);
  let path_segment = &url[IMAGE_URL_START_LENGTH..end_index];

  return path_segment.to_string();
}

fn download_card_image(client: &Client, card_id: &str, url: String, images_dir: &Path) -> Result<u8, Box<dyn std::error::Error>> {
  let image_path = url_to_filename(&url);
  println!("{image_path}");
  let file_path = images_dir.join(&image_path);

  if file_path.exists() {
    println!("Found cache, skipping: {}", file_path.display());
    return Ok(0);
  }

  fs::create_dir_all(file_path.parent().unwrap())?;

  let resp = client.get(&url).send()?;
    
  let res = fs::write(&file_path, resp.bytes()?);
  if res.is_err() {
    let err = res.err().unwrap();
    println!("ERROR: Failed to write card image for {}: {}", card_id, err.to_string());

    return Ok(2);
  }

  println!("Downloaded: {}", file_path.display());

  return Ok(1);
}

fn download_card_images(client: &Client, card_id: &str, image_uris: &ImageUris, images_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
  // match image_uris.small.clone() {
  //   Some(url) => {
  //     let _ = download_card_image(&client, card_id, url, images_dir);
  //     println!("Downloaded small image for {}", card_id);
  //   },
  //   None => {},
  // }

  // match image_uris.normal.clone() {
  //   Some(url) => {
  //     let _ = download_card_image(&client, card_id, url, images_dir);
  //     println!("Downloaded normal image for {}", card_id);
  //   },
  //   None => {},
  // }

  // match image_uris.large.clone() {
  //   Some(url) => {
  //     let _ = download_card_image(&client, card_id, url, images_dir);
  //     println!("Downloaded large image for {}", card_id);
  //   },
  //   None => {},
  // }

  match image_uris.png.clone() {
    Some(url) => {
      let _ = download_card_image(&client, card_id, url, images_dir);
      println!("Downloaded png image for {}", card_id);
    },
    None => {},
  }

  return Ok(());
}

fn fetch_card_images(client: &Client, cards: Vec<Card>, images_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
  println!("Downloading card images...");

  for (i, card) in cards.iter().enumerate() {
    if card.image_uris.is_some() {
      let image_uris = card.image_uris.as_ref().unwrap();
      let _ = download_card_images(&client, &card.id, image_uris, images_dir);
    } else {
      let card_faces = card.card_faces.as_ref().expect("card_faces should exist when image_uris are absent");

      for card_face in card_faces {
        if card_face.image_uris.is_some() {
          let image_uris = card_face.image_uris.as_ref().unwrap();
          let _ = download_card_images(&client, &card.id, image_uris, images_dir);
        }
      }
    }
    
    println!("STATUS: {}/{} cards downloaded", i + 1, cards.len());
  }

  return Ok(());
}

fn download_card_data(client: &Client, bulk_data: BulkDataResponse, images_dir: &PathBuf, output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
  let default_cards_url = bulk_data
      .data
      .into_iter()
      .find(|item| item.name == "Default Cards")
      .expect("Could not find default cards")
      .download_uri;

  println!("Default Cards URL: {}", default_cards_url);

  println!("Downloading bulk card data...");
  let cards: Vec<Card> = client.get(&default_cards_url).send()?.json()?;
  
  let bulk_data_filename = output_dir.join("bulk-data.json");

  let write_res = fs::write(bulk_data_filename, serde_json::to_string(&cards)?);

  if write_res.is_err() {
    let err = write_res.err().unwrap();
    println!("ERROR: Failed to write card data: {}", err.to_string());

    return Ok(());
  }

  let _ = fetch_card_images(&client, cards, images_dir);

  return Ok(());
}

fn download_card_rulings(client: &Client, bulk_data: BulkDataResponse, output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
  let rulings_url = bulk_data
      .data
      .into_iter()
      .find(|item| item.name == "Rulings")
      .expect("Could not find rulings")
      .download_uri;

  println!("Rulings URL: {}", rulings_url);

  println!("Downloading card ruling data...");
  let cards: Vec<Ruling> = client.get(&rulings_url).send()?.json()?;
  
  let ruling_data_filename = output_dir.join("ruling-data.json");

  let write_res = fs::write(ruling_data_filename, serde_json::to_string(&cards)?);

  if write_res.is_err() {
    let err = write_res.err().unwrap();
    println!("ERROR: Failed to write ruling data: {}", err.to_string());
  }

  return Ok(());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  dotenv().ok();

  // Setup
  let output_dir_str = var("SA_DATA_DIR").unwrap();
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
  println!("Fetching Scryfall bulk data list...");
  let bulk_data: BulkDataResponse = client
      .get("https://api.scryfall.com/bulk-data")
      .send()?
      .json()?;

  let _ = download_card_data(&client, bulk_data.clone(), &images_dir, output_dir);

  let _ = download_card_rulings(&client, bulk_data, output_dir);

  println!("Backup Complete.");
  return Ok(());
}
