use async_nats::{Client, ConnectError, PublishError, SubscribeError, Subscriber};
use bytes::Bytes;

#[derive(Debug)]
pub enum StreamError {
    SubscribeError(SubscribeError),
    ConnectionError(ConnectError),
    StreamPublishError(PublishError),
}
#[derive(Clone)]
pub struct NatsStream {
    client: Client,
}

impl NatsStream {
    pub async fn new(url: &str) -> Result<Self, StreamError> {
        match connect_to_server(url).await {
            Err(err) => Err(err),
            Ok(c) => Ok(Self { client: c }),
        }
    }

    pub fn get(&self) -> &Client {
        &self.client
    }
    /// Returns JetStream subscriber subscribed to subject
    pub async fn subscriber(&mut self, subject: &str) -> Result<Subscriber, StreamError> {
        match self.client.subscribe(subject.to_owned()).await {
            Ok(s) => Ok(s),
            Err(err) => Err(StreamError::SubscribeError(err)),
        }
    }
    pub async fn publish(&self, subject: &str, payload: Bytes) -> Result<(), StreamError> {
        match self.client.publish(subject.to_owned(), payload).await {
            Err(err) => Err(StreamError::StreamPublishError(err)),
            Ok(_) => Ok(()),
        }
    }
}
async fn connect_to_server(url: &str) -> Result<Client, StreamError> {
    match async_nats::connect(url).await {
        Err(err) => Err(StreamError::ConnectionError(err)),
        Ok(c) => Ok(c),
    }
}
