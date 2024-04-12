use async_nats::ConnectErrorKind::TimedOut;
use async_nats::{Client, ConnectError, SubscribeError, Subscriber};
use log::error;

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
            client: connect_to_server(url).await,
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
async fn connect_to_server(url: &str) -> Client {
    async_nats::connect(url).await.unwrap_or_else(|err| {
        error!("{err}");
        panic!("{url}");
    })
}
