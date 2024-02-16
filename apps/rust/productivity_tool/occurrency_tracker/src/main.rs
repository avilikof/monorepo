mod alert_handler;

use std::env;
use std::process::exit;

use alert_entity::AlertEntity;
use env_loader::load::load;
use kafka_driver::{KafkaClientConfig, KafkaConsumerClient};
use log::{debug, error, info};

#[tokio::main]
async fn main() {
    env_logger::init();
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
    loop {
        read_message(&client).await;
    }
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
        Some(msg) => match AlertEntity::from_bytes(&msg) {
            Ok(mut alert) => {
                debug!("{:?}", alert);
                debug!(
                    "{:?}",
                    String::from_utf8(alert.to_bytes().unwrap()).unwrap()
                );
            }
            Err(err) => {
                debug!("{:?}", err);
                debug!("{:?}", String::from_utf8(msg));
            }
        },
    }
}
