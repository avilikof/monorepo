use log::{debug, error, info};
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::error::{KafkaError, KafkaResult};
use rdkafka::message::{BorrowedMessage, Message};
use rdkafka::{Offset, TopicPartitionList};
use std::time::Duration;

pub struct KafkaConsumerClient {
    consumer: StreamConsumer,
}

pub struct KafkaClientConfig {
    bootstrap_server: String,
    user: String,
    pass: String,
    group_id: String,
}
impl KafkaClientConfig {
    pub fn new(bootstrap_server: String, user: String, pass: String, group_id: String) -> Self {
        Self {
            bootstrap_server,
            user,
            pass,
            group_id,
        }
    }
}
impl KafkaConsumerClient {
    pub fn connect(cfg: &KafkaClientConfig) -> Self {
        Self {
            consumer: create_kafka_consumer(cfg).unwrap(),
        }
    }

    pub fn subscribe(&self, topics: &[&str]) {
        debug!("subscription info: {:?}", self.consumer.subscription());
        if self.consumer.subscription().unwrap().count() != 0 {
            debug!("kafka consumer subscription exists skipping");
            return;
        }
        self.consumer.subscribe(topics).unwrap();
        info!(
            "no kafka consumer subscription found, subscribing to: {:?}",
            topics
        );
        // self.get_consumer_info(topics.first().to_owned().unwrap())
        self.get_current_offset();
    }

    pub fn unsubscribe(&self) {
        self.consumer.unsubscribe()
    }

    pub fn set_offset_to_beginning(&self) -> Result<(), KafkaError> {
        self.consumer
            .seek("write", 0, Offset::Beginning, Duration::from_secs(10))
    }
    pub fn set_offset_to_end(&self) -> Result<(), KafkaError> {
        self.consumer
            .seek("write", 0, Offset::End, Duration::from_secs(10))
    }

    fn subscribed_topics(&self) -> Vec<String> {
        self.consumer
            .subscription()
            .unwrap()
            .elements()
            .iter()
            .map(|element| element.topic().to_owned())
            .collect()
    }

    fn get_consumer_info(&self, topic: &str) {
        for t in self
            .consumer
            .fetch_metadata(Some(topic), Duration::from_secs(2))
            .unwrap()
            .topics()
        {
            info!("{:?}", t.name());
            for p in t.partitions() {
                info!("partition: {}", p.id());
            }
        }
    }

    pub async fn get_message(&self) -> Option<Vec<u8>> {
        match self.consumer.recv().await {
            Ok(msg) => msg.payload().map(|value| value.to_vec()),
            Err(err) => {
                error!("{err}");
                None
            }
        }
    }
    pub fn get_current_offset(&self) {
        let topics = self.subscribed_topics();
        let mut topic_partition_list = TopicPartitionList::new();
        topic_partition_list.add_partition(topics.first().unwrap().as_str(), 0);
    }
    pub async fn consume(&self) -> Result<BorrowedMessage, KafkaError> {
        match self.consumer.recv().await {
            Err(KafkaError::OffsetFetch(err)) => {
                error!("offsetFetch error");
                self.unsubscribe();
                Err(KafkaError::OffsetFetch(err))
            }
            Err(err) => {
                error!("kafka error: {}", err);
                self.unsubscribe();
                Err(err)
            }
            Ok(result) => Ok(result),
        }
    }
}

pub fn create_kafka_consumer(cfg: &KafkaClientConfig) -> KafkaResult<StreamConsumer> {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", &cfg.group_id)
        .set("client.id", "consumer_one")
        .set("bootstrap.servers", &cfg.bootstrap_server)
        .set("auto.offset.reset", "latest")
        .set("security.protocol", "SASL_SSL")
        .set("sasl.mechanisms", "SCRAM-SHA-256")
        .set("sasl.username", &cfg.user)
        .set("sasl.password", &cfg.pass)
        .create()?;
    Ok(consumer)
}