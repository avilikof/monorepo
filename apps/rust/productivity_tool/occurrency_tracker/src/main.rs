mod alert_handler;

use std::env;

use crate::alert_handler::AlertHandler;
use alert_entity::AlertEntity;
use env_loader::load::load;
use kafka_driver::{KafkaClientConfig, KafkaConsumerClient};
use log::{error, info};

#[tokio::main]
async fn main() {
    env_logger::init();
    // Load path is - monorepo/libs/rust/env_loader
    if let Err(_err) = load("../../../.env") {
        error!("failed to load .env values")
    }

    let config = set_kafka_config();
    let client = create_kafka_client(&config);
    let mut repo = repository::InMem::default();
    loop {
        read_message(&client, &mut repo).await;
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

async fn read_message(client: &KafkaConsumerClient, repo: &mut repository::InMem) {
    client.subscribe(&["alerts"]);
    match client.get_message().await {
        None => {}
        Some(msg) => match AlertEntity::from_bytes(&msg) {
            Ok(alert) => {
                let mut alert_handler = AlertHandler::init(&alert, repo);
                info!("{:?}", alert);
                info!("{}", alert_handler.alert_is_new());
            }
            Err(err) => {
                info!("{:?}", err);
                info!("{:?}", String::from_utf8(msg));
            }
        },
    }
}
