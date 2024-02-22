use env_loader::load::load;
use serde::{Deserialize, Serialize};
use serde_json;

use es_driver::ElasticClient;
use event_entity::EventEntity;
use kafka_driver::{KafkaClientConfig, KafkaConsumerClient};
use std::env;

#[derive(Deserialize, Debug)]
struct SearchResult<T> {
    hits: Hits<T>,
}

#[derive(Deserialize, Debug)]
struct Hits<T> {
    hits: Vec<Hit<T>>,
}

#[derive(Deserialize, Debug)]
struct Hit<T> {
    _source: T,
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    load(".env").expect("failed to load .env file");
    let url = env::var("ELASTIC_FULL_URL").unwrap();
    let config = get_kafka_config();
    let kafka_client = create_kafka_client(&config);
    let es_client = ElasticClient::new(&url);
    loop {
        kafka_client.subscribe(&["event"]);
        match kafka_client.get_message().await {
            None => {}
            Some(msg) => {
                if let Ok(mut event) = EventEntity::from_bytes(&msg) {
                    // es_client.index("test", &event).await.expect("Do something")
                    es_client
                        .index("test", event.as_bytes().unwrap())
                        .await
                        .expect("something bad");
                }
            }
        }
    }
}

fn get_kafka_config() -> KafkaClientConfig {
    let bootstrap = env::var("KAFKA_URL").unwrap();
    let user = env::var("KAFKA_USER").unwrap();
    let pass = env::var("KAFKA_PASS").unwrap();
    let group_id = env::var("GROUP_ID").unwrap();
    KafkaClientConfig::new(bootstrap, user, pass, group_id)
}

fn create_kafka_client(config: &KafkaClientConfig) -> KafkaConsumerClient {
    KafkaConsumerClient::connect(config)
}
