use crate::place_entity::Place;
use log::{debug, info};
use reqwest::{Error, StatusCode};
use std::thread;
use std::time::Duration;
use tokio::time::sleep;

pub async fn get_places(url: &str, code: &str) -> Result<String, Error> {
    // Make the HTTP GET request using reqwest
    let full_url = format!("{}/places/{}/forecasts", url, code);
    debug!("Fetching data from API");
    let resp = reqwest::get(&full_url).await?;

    // Check if the status is OK (200)
    match resp.status() {
        StatusCode::TOO_MANY_REQUESTS => {
            let timeout_sec = 60;
            log::info!("Sleeping for {}sec", timeout_sec);
            sleep(Duration::from_secs(timeout_sec)).await;
            log::info!("Resuming task..");
            let resp = reqwest::get(&full_url).await?;
            Ok(resp.text().await?)
        }
        StatusCode::OK => Ok(resp.text().await?),
        _ => {
            panic!("Error: Got {} response, url: {}", resp.status(), &full_url);
        }
    }
}
