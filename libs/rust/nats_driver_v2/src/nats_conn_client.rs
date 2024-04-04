use async_nats::{Client, SubscribeError, Subscriber};

#[derive(Debug)]
pub enum NatsConnClientError {
    SubscribeError(SubscribeError),
}
#[derive(Clone)]
pub struct NatsConnClient {
    client: Client,
}

impl NatsConnClient {
    pub async fn new(url: &str) -> Self {
        Self {
            client: async_nats::connect(url).await.unwrap(),
        }
    }
    pub fn _get(&self) -> &Client {
        &self.client
    }
    /// Returns JetStream subscriber subscribed to subject
    pub async fn get_subscriber(
        &mut self,
        subject: &str,
    ) -> Result<Subscriber, NatsConnClientError> {
        match self.client.subscribe(subject.to_owned()).await {
            Ok(s) => Ok(s),
            Err(err) => Err(NatsConnClientError::SubscribeError(err)),
        }
    }
}
