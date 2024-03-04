use alert_entity::AlertEntity;
use env_loader::load::load;
use kafka_driver::{KafkaClientConfig, KafkaProducerClient};
use log::{error, info};
use std::env;
use tokio::time;

#[tokio::main]
async fn main() {
    env_logger::init();
    match load(".env") {
        Ok(_) => {}
        Err(err) => error!("{err:?}"),
    }
    let kafka_producer = KafkaProducerClient::new(&KafkaClientConfig::new(
        env::var("KAFKA_URL").unwrap(),
        env::var("KAFKA_USER").unwrap(),
        env::var("KAFKA_PASS").unwrap(),
        env::var("GROUP_ID").unwrap(),
    ));
    let mut n = 0;
    loop {
        n += 1;
        let mut random_alert = AlertEntity::random();
        let alert_bytes = random_alert.as_bytes().unwrap();
        kafka_producer
            .send_message("test", "key", &alert_bytes)
            .await
            .unwrap();
        if n % 1000 == 0 {
            info!("{n}");
        }
        time::sleep(time::Duration::from_millis(10)).await;
    }
}
