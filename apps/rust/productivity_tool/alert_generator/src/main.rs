use env_loader::load::load;
use kafka_driver::{KafkaClientConfig, KafkaProducerClient};
use log::{debug, error, info};
use std::env;

#[tokio::main]
async fn main() {
    let mut kafka_addr: String = String::default();

    env_logger::init();
    match load(".env") {
        Ok(_) => {}
        Err(err) => error!("{err:?}"),
    }

    let _nats_url =
        env::var("NATS_URL").unwrap_or_else(|_| "nat://192.168.32.161:4222".to_string());
    match env::var("KAFKA_URL") {
        Ok(addr) => {
            debug!("Kafka addr: {}", &addr);
            kafka_addr = addr;
        }
        Err(err) => {
            panic!("{}", err)
        }
    };
    let kafka_client = KafkaProducerClient::new(KafkaClientConfig::new(
        kafka_addr,
        String::default(),
        String::default(),
        String::default(),
    ));

    // The URL to send the GET request to
    let url_places = "https://api.meteo.lt/v1/places";

    // Perform the HTTP GET request and get the response
    let places = get_places(url_places).await.unwrap();

    for place in places {
        if let Ok(data) = place.as_bytes() {
            kafka_client
                .send_message("test", "t", data.as_slice())
                .await
                .unwrap();
        } else {
            println!("Failure");
            continue;
        }

        // debug!("{}", place.code);
    }
    info!("Done");

    // Print each place code
}

use reqwest::Error;
use serde::{Deserialize, Serialize};

// Struct for Coordinates
#[derive(Debug, Deserialize, Serialize)]
struct Coordinates {
    latitude: f64,
    longitude: f64,
}

// Struct for Place
#[derive(Debug, Deserialize, Serialize)]
struct Place {
    code: String,
    name: String,
    administrativeDivision: String,
    countryCode: String,
    coordinates: Coordinates,
}

impl Place {
    pub fn as_bytes(&self) -> serde_json::Result<Vec<u8>> {
        serde_json::to_vec(self)
    }
}
// Function to get the places from the API
async fn get_places(url: &str) -> Result<Vec<Place>, Error> {
    // Make the HTTP GET request using reqwest
    let resp = reqwest::get(url).await?;

    // Check if the status is OK (200)
    if resp.status().is_success() {
        // Deserialize the JSON response into a Vec<Place>
        let places: Vec<Place> = resp.json().await?;
        Ok(places)
    } else {
        // If the status is not OK, return an error
        panic!("Error: Non-200 response");
    }
}
