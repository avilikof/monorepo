use bytes::Bytes;
use nats_driver_v2::connection::Connection;
// use tokio::time;

#[tokio::main]
async fn main() {
    env_logger::init();
    // let total_time = time::Instant::now();
    let nats_url =
        std::env::var("NATS_URL").unwrap_or_else(|_| "nats://192.168.32.161:4222".to_string());

    let conn: Connection;
    match nats_driver_v2::connection::Connection::new(&nats_url).await {
        Ok(c) => conn = c,
        Err(err) => panic!(err),
    }
    .await
    .expect("TODO: panic message");
    // let mut time_after_connected = time::Instant::now();

    conn.publish("test", Bytes::from("this is message"))?;
    conn.publish("test", Bytes::from("end"))?;

    println!("Time spent: {:?}", total_time.elapsed());
    println!(
        "Time without connection: {:?}",
        time_after_connected.elapsed()
    );
}
