use std::{env, time};

use log::{error, info};

use alert_entity::AlertEntity;
use env_loader::load::load;
use event_entity::EventEntity;
use kafka_driver::{KafkaClientConfig, KafkaConsumerClient, KafkaProducerClient};
use repository::InMemoryStorage;

use crate::handlers::alert_handler::OccurrenceHandler;
use crate::interfaces::repo_interface::RepoInterface;

use futures_util::stream::StreamExt;
use nats_driver_v2::NatsDriver; // Add this line to bring StreamExt into scope

mod handlers;
mod interfaces;

#[tokio::main]
async fn main() {
    env_logger::init();
    // Load path is - monorepo/libs/rust/env_loader
    if let Err(_err) = load(".env") {
        error!("failed to load .env values")
    }

    let mut nats_client = set_nats_client().await;
    let mut nats_subscriber = nats_client.get_subscriber("alerts").await.unwrap();

    // let mut nats_stream_sub_client = set_nats_client().await;
    // let nats_stream_pub_client = set_nats_client().await;
    // let mut nats_kv_client = set_nats_store().await;
    // nats_stream_sub_client.subscribe("alerts").await.unwrap();
    // let nats_subscription = nats_stream_sub_client.get_subscriber().unwrap();

    let ttl_as_string = env::var("TTL").unwrap();
    let ttl = time::Duration::from_secs(ttl_as_string.parse().unwrap());
    let mut new_repo = InMemoryStorage::new(Some(ttl));
    // let mut second_repo = new_repo.clone();
    // nats_kv_client
    //     .create_kv_storage("alerts")
    //     .await
    //     .expect("TODO: panic message");
    // let mut nats_kv_store = nats_kv_client.get_kv().unwrap();

    let kafka_thread = tokio::spawn(async move {
        start_kafka_client(&mut new_repo).await;
    });
    while let Some(message) = &nats_subscriber.next().await {
        let alert = AlertEntity::from_bytes(&message.payload).unwrap();
        let mut alert_handler = OccurrenceHandler::init(&alert, &mut nats_client);
        let mut event = alert_handler.occurrence_handling_flow().await;
        nats_client.
            .publish("events", bytes::Bytes::from(event.as_bytes().unwrap()))
            .await
            .unwrap();
    }
    kafka_thread.await.unwrap();
}

fn set_kafka_config() -> KafkaClientConfig {
    let bootstrap = env::var("KAFKA_URL").unwrap();
    let user = env::var("KAFKA_USER").unwrap();
    let pass = env::var("KAFKA_PASS").unwrap();
    let group_id = env::var("GROUP_ID").unwrap();
    KafkaClientConfig::new(bootstrap, user, pass, group_id)
}

async fn start_kafka_client(repo: &mut InMemoryStorage) {
    let config = set_kafka_config();
    let kafka_stream_client = create_kafka_client(&config);
    let kafka_producer = create_kafka_producer(&config);
    read_from_kafka(&kafka_stream_client, &kafka_producer, repo).await
}
async fn read_from_kafka(
    kafka_stream_client: &KafkaConsumerClient,
    kafka_producer: &KafkaProducerClient,
    repo: &mut InMemoryStorage,
) {
    let mut n = 0;
    loop {
        n += 1;
        let mut event = read_message(kafka_stream_client, repo).await;

        kafka_producer
            .send_message("event", "event", event.as_bytes().unwrap().as_slice())
            .await
            .expect("TODO: panic message");
        if n % 100 == 0 {
            info!("number of docs in storage: {}", &repo.get_count());
        }
    }
}

async fn set_nats_client() -> NatsDriver {
    let nats_url = env::var("NATS_URL").unwrap();
    NatsDriver::new(&nats_url).await
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
                alert_handler.occurrence_handling_flow().await
            }
            Err(err) => {
                error!("{:?}", err);
                error!("{:?}", String::from_utf8(msg));
                panic!("PANIC!!")
            }
        },
    }
}
