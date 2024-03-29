use alert_entity::AlertEntity;
use bytes::Bytes;
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

    let nats_url =
        env::var("NATS_URL").unwrap_or_else(|_| "nats://192.168.32.163:4222".to_string());

    let nats_client = nats_driver::NatsStreamClient::new(&nats_url).await;

    let mut n = 0;
    let mut start_time = time::Instant::now();
    let producer_kafka = tokio::spawn(async move {
        loop {
            n += 1;
            let mut random_alert = AlertEntity::random();
            let alert_bytes = random_alert.as_bytes().unwrap();
            kafka_produce(&alert_bytes, &kafka_producer).await;
            nats_client
                .publish("alerts", Bytes::from(alert_bytes.to_owned()))
                .await
                .expect("Failure for NATS");
            if n % 10 == 0 {
                info!("{n}");
                println!("{:?}", time::Instant::now() - start_time);
                start_time = time::Instant::now();
            }
            time::sleep(time::Duration::from_millis(100)).await;
        }
    });
    producer_kafka.await.unwrap();
}

async fn kafka_produce(message: &[u8], producer: &KafkaProducerClient) {
    producer
        .send_message("test", "alert", message)
        .await
        .expect("Failed write to kafka");
}
