use std::env;
use std::process::exit;

use env_loader::load::load;
use kafka_driver::{KafkaClientConfig, KafkaConsumerClient};
use log::error;

#[tokio::main]
async fn main() {
    // Load path is - monorepo/libs/rust/env_loader
    if let Err(_err) = load(".env") {
        error!("failed to load .env values")
    }
    match env::var("TEST") {
        Ok(val) => {
            println!("{val}")
        }
        Err(error) => {
            println!("{error}");
            exit(0)
        }
    }

    let config = set_kafka_config();
    let client = create_kafka_client(&config);
    read_message(&client).await;
}

fn set_kafka_config() -> KafkaClientConfig {
    let bootstrap = env::var("KAFKA_URL").unwrap();
    let user = env::var("KAFKA_USER").unwrap();
    let pass = env::var("KAFKA_PASS").unwrap();
    let group_id = env::var("GROUP_ID").unwrap();
    KafkaClientConfig::new(bootstrap, user, pass, group_id)
}

fn create_kafka_client(config: &KafkaClientConfig) -> KafkaConsumerClient {
    KafkaConsumerClient::connect(config)
}

async fn read_message(client: &KafkaConsumerClient) {
    client.subscribe(&["alerts"]);
    match client.get_message().await {
        None => {}
        Some(msg) => {
            println!("{:?}", String::from_utf8(msg))
        }
    }
}
