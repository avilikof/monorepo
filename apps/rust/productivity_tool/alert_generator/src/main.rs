use alert_entity::AlertEntity;
use bytes::Bytes;
use env_loader::load::load;
use futures::StreamExt;
use kafka_driver::{KafkaClientConfig, KafkaProducerClient};
use log::{error, info};
use std::env;
use std::str::from_utf8;
use tokio::time;

#[tokio::main]
async fn main() {
    env_logger::init();
    match load(".env") {
        Ok(_) => {}
        Err(err) => error!("{err:?}"),
    }
    // let kafka_producer = KafkaProducerClient::new(&KafkaClientConfig::new(
    //     env::var("KAFKA_URL").unwrap(),
    //     env::var("KAFKA_USER").unwrap(),
    //     env::var("KAFKA_PASS").unwrap(),
    //     env::var("GROUP_ID").unwrap(),
    // ));

    let nats_url =
        env::var("NATS_URL").unwrap_or_else(|_| "nats://192.168.32.163:4222".to_string());

    let nats_client = nats_driver::NatsClient::new(&nats_url).await;

    let mut n = 0;
    let mut start_time = time::Instant::now();
    let producer = tokio::spawn(async move {
        loop {
            n += 1;
            let mut random_alert = AlertEntity::random();
            let alert_bytes = random_alert.as_bytes().unwrap();
            // kafka_produce(&alert_bytes, &kafka_producer).await;
            nats_client
                .publish("greet.joe", Bytes::from(alert_bytes.to_owned()))
                .await
                .expect("Failure for NATS");
            if n % 1000 == 0 {
                info!("{n}");
                println!("{:?}", time::Instant::now() - start_time);
                // time::sleep(Duration::from_secs(5)).await;
                start_time = time::Instant::now();
            }
            time::sleep(time::Duration::from_micros(10)).await;
        }
    });
    let reader = tokio::spawn(async {
        let nats_url = env::var("NATS_URL").unwrap();

        let client = async_nats::connect(nats_url).await.unwrap();

        let mut subscription = client.subscribe("greet.*").await.unwrap();
        while let Some(message) = subscription.next().await {
            println!(
                "{:?} received on {:?}",
                from_utf8(&message.payload),
                &message.subject
            );
        }
        println!("Done");
    });
    // time::sleep(time::Duration::from_secs(15)).await;
    reader.await.unwrap();
    producer.await.unwrap();
}

async fn kafka_produce(message: &[u8], producer: &KafkaProducerClient) {
    producer
        .send_message("test", "alert", message)
        .await
        .expect("Failed write to kafka");
}
