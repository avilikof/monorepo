use std::env;
use env_loader::load::load;
use kafka_driver::{KafkaClientConfig, KafkaConsumerClient};
use log::error;

#[tokio::main]
async fn main() {
    if let Err(_err) = load(".env") { error!("failed to load .env values") }
    let name = env::var("TEST").unwrap();

    println!("Hello, {}", name);

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
       None => {},
       Some(msg) => {
           println!("{:?}", String::from_utf8(msg))
       },
   }
}
