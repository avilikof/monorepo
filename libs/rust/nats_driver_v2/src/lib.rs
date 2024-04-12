mod nats_conn_client;
mod nats_context;
mod nats_store;

use async_nats::jetstream::context::{
    CreateStreamError, GetStreamError, PublishAckFuture, PublishError,
};
use async_nats::jetstream::kv::Store;
use async_nats::jetstream::stream::{Config, Stream};
use async_nats::jetstream::Context;
use async_nats::Subscriber;

use bytes::Bytes;
use log::warn;

use crate::nats_conn_client::{NatsConnClient, NatsConnClientError};
use crate::nats_context::NatsContext;
use crate::nats_store::{NatsStorage, NatsStoreError};

#[derive(Debug)]
pub enum NatsDriverError {
    StoreError(NatsStoreError),
    CreateStreamError(CreateStreamError),
    GetStreamError(GetStreamError),
    ContextNotExists(String),
    ErrorSubscribing(NatsConnClientError),
    FailedPublishingToJetStream(PublishError),
    StorePullKVError(NatsStoreError),
}
pub struct NatsDriver {
    client: NatsConnClient,
    context: NatsContext,
    store: NatsStorage,
}

impl NatsDriver {
    pub async fn new(url: &str) -> Self {
        Self {
            client: NatsConnClient::new(url).await,
            context: NatsContext::default(),
            store: NatsStorage::default(),
        }
    }
    pub async fn _init_client(&mut self, url: &str) {
        self.client = NatsConnClient::new(url).await
    }
    pub async fn init_context(&mut self) {
        self.context = NatsContext::_new(self.client._get()).await
    }
    pub async fn init_store(&mut self, bucket_name: &str) {
        if self.context.get().is_none() {
            let err = NatsDriverError::ContextNotExists(
                "context not found, creating new one".to_string(),
            );
            warn!("{:?}", err);
            self.init_context().await;
        };
        self.store = NatsStorage::new(&self.context, bucket_name).await
    }

    pub fn get_client(&self) -> &NatsConnClient {
        &self.client
    }
    pub fn _get_context(&self) -> &NatsContext {
        &self.context
    }
    pub fn _get_store(&self) -> &NatsStorage {
        &self.store
    }
    pub fn jetstream(&self) -> &Context {
        self.context.get().unwrap()
    }
    pub async fn kv(&mut self, bucket_name: &str) -> &Store {
        if self.store.get().is_err() {
            self.init_store(bucket_name).await;
        };
        self.store.get().unwrap()
    }
    pub async fn create_stream<S>(&self, config: S) -> Result<Stream, NatsDriverError>
    where
        Config: From<S>,
    {
        match self.jetstream().create_stream(config).await {
            Err(err) => Err(NatsDriverError::CreateStreamError(err)),
            Ok(s) => Ok(s),
        }
    }
    pub async fn get_stream<T: AsRef<str>>(&self, stream: T) -> Result<Stream, NatsDriverError> {
        match self.jetstream().get_stream(stream).await {
            Err(err) => Err(NatsDriverError::GetStreamError(err)),
            Ok(s) => Ok(s),
        }
    }
    pub async fn get_subscriber(&mut self, subject: &str) -> Result<Subscriber, NatsDriverError> {
        match self.client.get_subscriber(subject).await {
            Ok(s) => Ok(s),
            Err(err) => Err(NatsDriverError::ErrorSubscribing(err)),
        }
    }
    pub async fn publish(
        &mut self,
        subject: &str,
        message: Bytes,
    ) -> Result<PublishAckFuture, NatsDriverError> {
        match self.context.get() {
            None => Err(NatsDriverError::ContextNotExists(
                "Context doesnt exits".to_owned(),
            )),
            Some(c) => match c.publish(subject.to_owned(), message).await {
                Ok(ack) => Ok(ack),
                Err(err) => Err(NatsDriverError::FailedPublishingToJetStream(err)),
            },
        }
    }
    pub async fn pull_kv(&self, key: &str) -> Result<Option<Bytes>, NatsDriverError> {
        match self.store.pull_kv(key).await {
            Ok(msg) => Ok(msg),
            Err(err) => Err(NatsDriverError::StorePullKVError(err)),
        }
    }
    pub async fn push_kv(&mut self, key: &str, value: Bytes) -> Result<u64, NatsDriverError> {
        match self.store.push_kv(key, value).await {
            Ok(i) => Ok(i),
            Err(err) => Err(NatsDriverError::StoreError(err)),
        }
    }
    pub async fn update_kv(&mut self, key: &str, value: Bytes) -> Result<u64, NatsDriverError> {
        match self.store.update_kv(key, value).await {
            Ok(i) => Ok(i),
            Err(err) => Err(NatsDriverError::StoreError(err)),
        }
    }
    pub async fn delete_kv(&mut self, key: &str) -> Result<(), NatsDriverError> {
        match self.store.delete_kv(key).await {
            Ok(_) => Ok(()),
            Err(err) => Err(NatsDriverError::StoreError(err)),
        }
    }
}
