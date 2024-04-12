use async_nats::jetstream::stream::Config;
use async_nats::jetstream::Context;
use async_nats::{jetstream, Client};

pub enum NatsContextError {}
#[derive(Default, Clone)]
pub struct NatsContext {
    context: Option<Context>,
    config: Option<Config>,
}
impl NatsContext {
    pub async fn _new(client: &Client) -> Self {
        Self {
            context: Some(jetstream::new(client.to_owned())),
            config: None,
        }
    }
    pub fn get(&self) -> Option<&Context> {
        self.context.as_ref()
    }
}
