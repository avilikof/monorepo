use crate::config::{get_system_variable, init_env};
use crate::debug_messages::{announce_initialization_finish, announce_initialization_start};
use futures::future::join_all;
use kafka_driver::{KafkaClientConfig, KafkaProducerClient};
use kafka_driver_v2::ConsumerClient;
use log::{debug, info};
use place_entity::Place;
use reqwest::Error;
use std::future::Future;
use std::time::Instant;

mod config;
mod debug_messages;
mod place_entity;
mod redis;
mod web_client;

#[tokio::main]
async fn main() {
    init_env();
    announce_initialization_start();
    let mut redis_handler = redis::RedisHandler::new("redis://192.168.32.161:32779");
    let meteo_api_base_url = get_system_variable("METEO_API_BASE_URL");

    let keys = &redis_handler.get_keys();
    let mut m: u128 = 0;
    let start_time = Instant::now();
    for (n, key) in (0_u128..).zip(keys.iter()) {
        m = n;
        match redis_handler.get(key) {
            None => continue,
            Some(value) => {
                log::debug!("{}", String::from_utf8(value.clone()).unwrap());
                match serde_json::from_slice::<Place>(value.as_slice()) {
                    Ok(place) => {
                        log::debug!("{:?}", place);
                        match web_client::get_places(&meteo_api_base_url, &place.code).await {
                            Ok(resp) => {
                                debug!("{resp:?}")
                            }
                            Err(err) => log::warn!("{err}"),
                        }
                    }
                    Err(err) => {
                        log::warn!("{:?}\nValue - {}", err, String::from_utf8(value).unwrap())
                    }
                }
            }
        }
    }
    log::info!("Number of keys: {}", m);
    log::info!("Time took: {:?}", start_time.elapsed());
    return;
    let kafka_address: String = get_system_variable("KAFKA_URL");
    let kafka_producer = init_kafka_producer(&kafka_address);
    let kafka_consumer = ConsumerClient::new(&kafka_address, "group_1").unwrap();
    announce_initialization_finish();
    // match get_places(&meteo_api_base_url).await {
    //     Ok(response) => {
    //         store_response(&response, &kafka_producer).await;
    //     }
    //     Err(_err) => {
    //         panic!("{_err:?}")
    //     }
    // }
    read_from_kafka(&kafka_consumer).await;
}

async fn store_response(response: &Vec<Place>, storage: &KafkaProducerClient) {
    debug!("{response:?}");
    info!("Start writing to kafka..");
    write_to_kafka(response, storage).await;
}

fn init_kafka_producer(kafka_bootstrap_addr: &str) -> KafkaProducerClient {
    debug!("Initializing kafka producer");
    KafkaProducerClient::new(KafkaClientConfig::default(kafka_bootstrap_addr.to_string()))
}

// async fn get_places(url: &str) -> Result<Vec<Place>, Error> {
//     // Make the HTTP GET request using reqwest
//     let full_url = format!("{}/places", url);
//     info!("Fetching data from API");
//     let resp = reqwest::get(full_url).await?;
//
//     // Check if the status is OK (200)
//     if resp.status().is_success() {
//         // Deserialize the JSON response into a Vec<Place>
//         let places: Vec<Place> = resp.json().await?;
//         Ok(places)
//     } else {
//         // If the status is not OK, return an error
//         let _status = resp.status().to_string();
//         panic!("Error: Got {_status} response");
//     }
// }

async fn write_to_kafka(data: &[Place], kafka_producer_client: &KafkaProducerClient) {
    // TODO use generic way to send data to kafka
    let start_time = Instant::now();
    // Collect all the send_message futures
    let send_futures = data.iter().map(|place| {
        kafka_producer_client.send_message("test", "alert", place.as_bytes().unwrap())
    });

    // Await all futures concurrently
    let results = join_all(send_futures).await;

    // Check for errors
    for result in results {
        if let Err(_err) = result {
            panic!("{_err}");
        }
    }
    let duration = start_time.elapsed();
    info!("Done writing. It took {:?} to finish the loop.", duration);
}

async fn read_from_kafka(consumer: &ConsumerClient) {
    consumer.subscribe(&["test"]);
    loop {
        match consumer.get_message().await {
            Ok(message) => match message {
                None => break,
                Some(payload) => {
                    println!("{}", String::from_utf8(payload).unwrap());
                }
            },
            Err(err) => panic!("{:?}", err),
        }
    }
}
