mod handlers;
mod interfaces;

use std::{env, time};

use crate::handlers::alert_handler::AlertHandler;
use crate::interfaces::repo_interface::RepoInterface;
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
    let ttl_as_string = env::var("TTL").unwrap();
    let ttl = time::Duration::from_secs(ttl_as_string.parse().unwrap());
    let mut new_repo = repository::InMemoryStorage::new(Some(ttl));
    let mut n = 0;
    loop {
        n += 1;
        read_message(&client, &mut new_repo).await;
        if n % 100 == 0 {
            info!("number of docs in storage: {}", &new_repo.get_count());
            // info!("list of all keys: {:?}", &new_repo.get_keys());
            new_repo.cleanup();
        }
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

async fn read_message<R>(client: &KafkaConsumerClient, repo: &mut R)
where
    R: RepoInterface,
{
    client.subscribe(&["alerts"]);
    match client.get_message().await {
        None => {}
        Some(msg) => match AlertEntity::from_bytes(&msg) {
            Ok(alert) => {
                let mut alert_handler = AlertHandler::init(&alert, repo);
                alert_handler.occurrence_handling_flow();
            }
            Err(err) => {
                info!("{:?}", err);
                info!("{:?}", String::from_utf8(msg));
            }
        },
    }
}
