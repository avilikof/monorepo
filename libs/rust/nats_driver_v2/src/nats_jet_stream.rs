use async_nats::jetstream::stream::Config;
use async_nats::jetstream::Context;
use async_nats::{jetstream, Client};

pub enum JetStreamError {}
#[derive(Default, Clone)]
pub struct JetStream {
    context: Option<Context>,
    config: Option<Config>,
}
impl JetStream {
    pub async fn new(client: &Client) -> Self {
        Self {
            context: Some(jetstream::new(client.to_owned())),
            config: None,
        }
    }
    pub fn get(&self) -> Option<&Context> {
        self.context.as_ref()
    }
}
