mod archiver;
mod json_stream;
mod types;

use std::{env::var, str::FromStr, thread};

use chrono::Utc;
use cron::Schedule;
use dotenv::dotenv;
use log::info;
use progress_bar::{init_logger};
use crate::archiver::archive_scryfall;

#[tokio::main]
async fn main() {
  dotenv().ok();
  
  init_logger().unwrap();

  info!("Thread: Starting Archiver...");

  // Run Archiver initally
  let _ = archive_scryfall().await;
  
  let archive_schedule = var("SA_ARCHIVE_SCHEDULE").unwrap_or(String::from("0 2 1 * * * *"));
  let schedule = Schedule::from_str(&archive_schedule).expect("Failed to parse CRON expression");

  for datetime in schedule.upcoming(Utc).take(1) {
    let now = Utc::now();
    let until = datetime - now;
    thread::sleep(until.to_std().unwrap());

    let _ = archive_scryfall().await;
  }
}
