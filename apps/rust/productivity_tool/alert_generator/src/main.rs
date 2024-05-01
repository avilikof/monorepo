use alert_entity::AlertEntity;
use bytes::Bytes;
use env_loader::load::load;
use kafka_driver::{KafkaClientConfig, KafkaProducerClient};
use log::{error, info};
use nats_driver_v2::{NatsDriver, NatsDriverError};
use std::env;
use tokio::time;

#[tokio::main]
async fn main() {
    env_logger::init();
    match load(".env") {
        Ok(_) => {}
        Err(err) => error!("{err:?}"),
    }

    let nats_url = env::var("NATS_URL").unwrap_or_else(|_| "nat://192.168.32.161:4222".to_string());

    let mut n = 0;
    let mut start_time = time::Instant::now();
    let nats_client = NatsDriver::new(&nats_url).await;
    loop {
        n += 1;
        let mut random_alert = AlertEntity::random();
        nats_client
            .nats_stream_publish("new_alerts", random_alert.as_bytes().unwrap())
            .await
            .unwrap();
        if n % 100000 == 0 {
            info!("{n}");
            println!("{:?}", time::Instant::now() - start_time);
            start_time = time::Instant::now();
        }
        // time::sleep(time::Duration::from_millis(10)).await;
    }
}

async fn kafka_produce(message: &[u8], producer: &KafkaProducerClient) {
    producer
        .send_message("test", "alert", message)
        .await
        .expect("Failed write to kafka");
}
