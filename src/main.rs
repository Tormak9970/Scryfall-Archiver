mod archiver;

use std::{env::var, str::FromStr, thread};

use chrono::Utc;
use cron::Schedule;
use dotenv::dotenv;
use log::info;
use crate::archiver::archive_scryfall;


fn main() -> Result<(), Box<dyn std::error::Error>> {
  dotenv().ok();
  
  pretty_env_logger::init_timed();

  std::thread::spawn(move || {
    info!("Thread: Starting Archiver...");
    
    let archive_schedule = var("SA_ARCHIVE_SCHEDULE").unwrap();

    let schedule = Schedule::from_str(&archive_schedule).expect("Failed to parse CRON expression");

    for datetime in schedule.upcoming(Utc).take(1) {
      let now = Utc::now();
      let until = datetime - now;
      thread::sleep(until.to_std().unwrap());

      let _ = archive_scryfall();
    }
  });
  
  return Ok(());
}
