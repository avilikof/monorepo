use async_nats::Client;
use std::future::IntoFuture;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use async_nats::jetstream::{stream::StorageType, Context};

use async_nats::jetstream::kv::Store;
use async_nats::jetstream::stream::Config;
use bytes::Bytes;
use futures_util::StreamExt;

use log::{error, info};
use nats_driver_v2::NatsDriver;
use tokio::sync::Semaphore;
use tokio::time;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    env_logger::init();
    let total_time = time::Instant::now();
    let nats_url =
        std::env::var("NATS_URL").unwrap_or_else(|_| "nats://192.168.32.161:4222".to_string());

    let mut nats_client= NatsDriver::new(&nats_url).await;

    let mut stream_subscriber = nats_client.get_subscriber("alerts").await.unwrap();
    
    let time_after_connected = time::Instant::now();

    let task = tokio::spawn(async move {
        while let Some(msg) = &stream_subscriber.next().await {
            println!("{:?}",&msg.payload);
            if &msg.payload == "end" {
                break
            }
        }
    });
    nats_client.nats_stream_publish("alerts", "this is it".as_bytes()).await.unwrap();
    nats_client.nats_stream_publish("alerts", "end".as_bytes()).await.unwrap();
    task.await.unwrap();
    println!("Time spent: {:?}", total_time.elapsed());
    println!("Time without connection: {:?}", time_after_connected.elapsed());

}

//     client.init_jet_stream().await;
//     client.publish("test", "message".into()).await.expect("TODO: panic message");
//
//
//     let mut subscr= client.get_subscriber("alerts").await.expect("Im done");
//     info!("Message published");
//     sleep(time::Duration::from_secs(5)).await;
//     while let Some(msg) = &subscr.next().await {
//         println!("{:?}", &msg.payload)
//     }
//
//     // client.init_store("alerts_storage").await;
//
//     let stream_config = Config {
//         name: "EVENTS".to_string(),
//         subjects: vec!["events.>".to_string()],
//         storage: StorageType::File,
//         ..Default::default()
//     };
//
//     // stream_config.storage = StorageType::File;
//
//     let _stream = match client.create_stream(stream_config.clone()).await {
//         Ok(s) => s,
//         Err(_) => client.jetstream().get_stream("EVENTS").await.unwrap(),
//     };
//     info!("created the stream");
//
//     // let start_time = time::Instant::now();
//
//     const MESSAGE_COUNT: i32 = 10;
//     sync_producer(MESSAGE_COUNT, client.jetstream().to_owned(), 0_i32).await;
//
//     _async_producer(0, client.jetstream().to_owned(), 0_i32).await;
//     info!("writing in mem");
//     for _ in 0..MESSAGE_COUNT {
//         store_kv(client.kv("alert_state_in_mem").await).await;
//     }
//     info!("writing on disk");
//     for _ in 0..MESSAGE_COUNT {
//         store_kv(client.kv("alert_state").await).await;
//     }
//     let semaphore = Arc::new(Semaphore::new(5));
//     _optimized_async_producer(client.jetstream().to_owned(), 0_usize, 1, semaphore).await;
//
//     // println!("{:#?}", stream.info().await?);
//
//     // const MAX_MESSAGES: i64 = 1000000000;
//     // stream_config.max_messages = MAX_MESSAGES;
//     // jetstream.update_stream(stream_config.clone()).await?;
//     // println!("set max messages to {MAX_MESSAGES}");
//     //
//     //
//     // println!("{:#?}", stream.info().await?);
//     //
//     //
//     // const MAX_BYTES:i64 = 30000000000;
//     // stream_config.max_bytes = MAX_BYTES;
//     // jetstream.update_stream(stream_config.clone()).await?;
//     // println!("set max bytes to {MAX_BYTES}");
//     //
//     //
//     // println!("{:#?}", stream.info().await?);
//     //
//     //
//     // const DURATION_SECONDS: u64 = 300;
//     // stream_config.max_age = Duration::from_secs(DURATION_SECONDS);
//     // jetstream.update_stream(stream_config.clone()).await?;
//     // println!("set max age to {DURATION_SECONDS} seconds");
//     //
//     //
//     // println!("{:#?}", stream.info().await?);
//
//     // println!("sleeping one second...");
//     // tokio::time::sleep(Duration::from_secs(1)).await;
//
//     // println!("{:#?}", stream.info().await?);
//     //
//
//     info!("Total time spent: {:?}", total_time.elapsed());
//     Ok(())
// }
//
// async fn sync_producer(message_count: i32, jeatstream: Context, thread_id: i32) {
//     info!("Starting sync producer thread :: {thread_id}");
//     let start_time = time::Instant::now();
//     for _ in 0..=message_count {
//         let mut alert = alert_entity::AlertEntity::random();
//         jeatstream
//             .publish(
//                 "events.page_loaded",
//                 bytes::Bytes::from(alert.as_bytes().unwrap()),
//             )
//             .await
//             .expect("TODO: panic message");
//     }
//     info!(
//         "Thread :: {thread_id} :: Finished Sync production of {} messages, took {:?}",
//         message_count,
//         start_time.elapsed()
//     )
// }
//
// async fn _async_producer(message_count: i32, jetsream: Context, thread_id: i32) {
//     if message_count == 0 {
//         return;
//     }
//     info!("Starting Async producer thread :: {thread_id}");
//     let start_time = time::Instant::now();
//     let mut acks = Vec::new();
//     for _ in 0..=message_count {
//         acks.push(
//             jetsream
//                 .publish("events.input_changed", "".into())
//                 .await
//                 .unwrap()
//                 .into_future(),
//         );
//     }
//
//     match futures::future::try_join_all(acks).await {
//         Ok(_acks) => info!(
//             "Thread :: {thread_id} :: Finished Async published {message_count} messages in {:?}",
//             start_time.elapsed()
//         ),
//         Err(err) => panic!("failed to ack all messages: {}", err),
//     }
// }
//
// async fn get_client(nats_url: String) -> Client {
//     async_nats::connect(nats_url).await.unwrap()
// }
//
// async fn _optimized_async_producer(
//     jetstream: Context,
//     message_count: usize,
//     thread_id: i32,
//     semaphore: Arc<Semaphore>,
// ) {
//     info!("Starting optimized async producer thread :: {thread_id}");
//     let start_time = time::Instant::now();
//
//     let acks = Arc::new(Mutex::new(Vec::new()));
//     for _ in 0..message_count {
//         let permit = semaphore
//             .clone()
//             .acquire_owned()
//             .await
//             .expect("Failed to acquire semaphore permit");
//         let js_clone = jetstream.clone();
//         let acks_clone = acks.clone();
//
//         tokio::spawn(async move {
//             let ack = js_clone.publish("events.input_changed", "".into()).await;
//             drop(permit); // Release the permit as soon as the message is published
//             if let Ok(ack) = ack {
//                 acks_clone.lock().unwrap().push(ack);
//             }
//         });
//     }
//
//     // Ensure all messages have been acknowledged
//     while Arc::strong_count(&acks) > 1 {
//         time::sleep(Duration::from_millis(100)).await;
//     }
//
//     info!(
//         "Optimized :: Thread :: {thread_id} :: Finished publishing {message_count} messages in {:?}",
//         start_time.elapsed()
//     );
// }
//
// async fn store_kv(kv: &Store) {
//     let mut alert = alert_entity::AlertEntity::random();
//     let id = alert.get_alert_id().to_owned();
//     let alert_bytes = alert.as_bytes().unwrap();
//     let alert_fot_nats = bytes::Bytes::from(alert_bytes);
//     let write_time = time::Instant::now();
//     if kv.put(id.clone(), alert_fot_nats).await.is_err() {
//         error!("failed to write to kv");
//     }
//     let read_time = time::Instant::now();
//     kv.get(id.clone()).await.unwrap().unwrap();
//     info!(
//         "write time: {:?} :: read time: {:?}",
//         write_time.elapsed(),
//         read_time.elapsed()
//     );
// }
//
// async fn put_to_stream(stream: &Context, msg: Bytes) {}
