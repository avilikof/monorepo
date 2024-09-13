use crate::nats_stream::StreamError;
use async_nats::{Client, ConnectError, UnsubscribeError};
use bytes::Bytes;
use std::fmt::Error;
use std::future::Future;

#[derive(Debug)]
pub struct Connection {
    client: async_nats::Client,
}

pub enum ConnectionError {
    ConnectErr,
    PubError,
}

impl Connection {
    pub async fn new(url: &str) -> Result<Self, ConnectionError> {
        match Self::connect_to_nats_server(url).await {
            Ok(c) => Ok(Self { client: c }),
            Err(err) => Err(ConnectionError::ConnectErr),
        }
    }
    async fn connect_to_nats_server(url: &str) -> Result<async_nats::Client, ConnectError> {
        match async_nats::connect(url).await {
            Ok(c) => Ok(c),
            Err(err) => Err(err),
        }
    }
    pub async fn publish(&self, subject: &str, payload: Bytes) -> Result<(), ConnectionError> {
        match self.client.publish(subject, payload).await {
            Ok(_) => {
                self.client.flush().await.expect("error during flush");
                Ok(())
            }
            Err(err) => Err(ConnectionError::PubError),
        }
    }
}
