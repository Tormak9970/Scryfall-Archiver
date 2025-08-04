mod archiver;

use dotenv::dotenv;
use crate::archiver::archive_scryfall;


fn main() -> Result<(), Box<dyn std::error::Error>> {
  dotenv().ok();

  let _ = archive_scryfall();
  
  return Ok(());
}
