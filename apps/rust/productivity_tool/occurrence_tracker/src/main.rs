use std::{env, time};

use log::{error, info};

use alert_entity::AlertEntity;
use env_loader::load::load;
use event_entity::EventEntity;
use kafka_driver::{KafkaClientConfig, KafkaConsumerClient, KafkaProducerClient};
use nats_driver::NatsStreamClient;
use repository::InMemoryStorage;

use crate::handlers::alert_handler::OccurrenceHandler;
use crate::interfaces::repo_interface::RepoInterface;

mod handlers;
mod interfaces;

#[tokio::main]
async fn main() {
    env_logger::init();
    // Load path is - monorepo/libs/rust/env_loader
    if let Err(_err) = load(".env") {
        error!("failed to load .env values")
    }

    let config = set_kafka_config();
    let kafka_stream_client = create_kafka_client(&config);
    let nats_stream_client = set_nats_client().await;
    let kafka_producer = create_kafka_producer(&config);
    let ttl_as_string = env::var("TTL").unwrap();
    let ttl = time::Duration::from_secs(ttl_as_string.parse().unwrap());
    let mut new_repo = InMemoryStorage::new(Some(ttl));
    let mut n = 0;
    loop {
        n += 1;
        let mut event = read_message(&kafka_stream_client, &mut new_repo).await;
        kafka_producer
            .send_message("event", "event", event.as_bytes().unwrap().as_slice())
            .await
            .expect("TODO: panic message");
        nats_stream_client
            .publish("events", bytes::Bytes::from(event.as_bytes().unwrap()))
            .await
            .unwrap();
        if n % 100 == 0 {
            info!("number of docs in storage: {}", &new_repo.get_count());
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

async fn set_nats_client() -> NatsStreamClient {
    let nats_url = env::var("NATS_URL").unwrap();
    NatsStreamClient::new(&nats_url).await
}
fn create_kafka_client(config: &KafkaClientConfig) -> KafkaConsumerClient {
    KafkaConsumerClient::connect(config)
}
fn create_kafka_producer(config: &KafkaClientConfig) -> KafkaProducerClient {
    KafkaProducerClient::new(config)
}

async fn read_message<R>(client: &KafkaConsumerClient, repo: &mut R) -> EventEntity
where
    R: RepoInterface,
{
    client.subscribe(&["alerts"]);
    match client.get_message().await {
        None => todo!(),
        Some(msg) => match AlertEntity::from_bytes(&msg) {
            Ok(alert) => {
                let mut alert_handler = OccurrenceHandler::init(&alert, repo);
                alert_handler.occurrence_handling_flow()
            }
            Err(err) => {
                error!("{:?}", err);
                error!("{:?}", String::from_utf8(msg));
                panic!("PANIC!!")
            }
        },
    }
}
