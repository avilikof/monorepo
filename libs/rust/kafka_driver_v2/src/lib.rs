use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::error::KafkaError;
use rdkafka::message::{BorrowedMessage, Message};

pub struct ConsumerClient {
    consumer: StreamConsumer,
}

impl ConsumerClient {
    pub fn new(brokers: &str, group_id: &str) -> Result<Self, KafkaError> {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("group.id", group_id)
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "true")
            .set("auto.offset.reset", "earliest") // or "latest"
            .create()?;
        Ok(Self { consumer })
    }
    pub fn subscribe(&self, topics: &[&str]) {
        self.consumer.subscribe(topics).unwrap()
    }
    pub async fn get_message(&self) -> Result<Option<Vec<u8>>, KafkaError> {
        match self.consumer.recv().await {
            Ok(payload) => Ok(self.extract_payload(payload)),
            Err(err) => Err(err),
        }
    }
    fn extract_payload(&self, borrowed_message: BorrowedMessage) -> Option<Vec<u8>> {
        borrowed_message.payload().map(|payload| payload.to_vec())
    }
}
