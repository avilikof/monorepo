mod nats_jet_stream;
mod nats_store;
mod nats_stream;

use async_nats::jetstream::context::{
    CreateStreamError, GetStreamError, PublishAckFuture, PublishError,
};
use async_nats::jetstream::kv::Store;
use async_nats::jetstream::stream::{Config, Stream};
use async_nats::jetstream::Context;
use async_nats::rustls::internal::msgs::base::Payload;
use async_nats::Subscriber;

use bytes::Bytes;
use log::warn;

use crate::nats_jet_stream::JetStream;
use crate::nats_store::{NatsKVStorage, NatsStoreError};
use crate::nats_stream::{NatsStream, StreamError};

#[derive(Debug)]
pub enum NatsDriverError {
    StoreError(NatsStoreError),
    CreateStreamError(CreateStreamError),
    GetStreamError(GetStreamError),
    ContextNotExists(String),
    ErrorSubscribing(StreamError),
    FailedPublishingToJetStream(PublishError),
    StorePullKVError(NatsStoreError),
    NatsStreamError(StreamError),
    NatsStreamPublishError(StreamError),
}
pub struct NatsDriver {
    nats_stream: NatsStream,
    jet_stream: Option<JetStream>,
    kv_store: Option<NatsKVStorage>,
}

impl NatsDriver {
    pub async fn new(url: &str) -> Self {
        Self {
            nats_stream: NatsStream::new(url).await.unwrap(), // TODO implement proper handling
            jet_stream: None,
            kv_store: None,
        }
    }
    pub async fn init_nats_stream(&mut self, url: &str) {
        match NatsStream::new(url).await {
            Err(err) => panic!("{:?}", err),
            Ok(s) => self.nats_stream = s,
        }
    }

    pub async fn get_subscriber(&mut self, subject: &str) -> Result<Subscriber, NatsDriverError> {
        match self.nats_stream.subscriber(subject).await {
            Err(err) => Err(NatsDriverError::NatsStreamError(err)),
            Ok(subscr) => Ok(subscr),
        }
    }

    pub async fn nats_stream_publish(
        &self,
        subject: &str,
        payload: Vec<u8>,
    ) -> Result<(), NatsDriverError> {
        match self
            .nats_stream
            .publish(subject, bytes::Bytes::copy_from_slice(&payload))
            .await
        {
            Err(err) => Err(NatsDriverError::NatsStreamPublishError(err)),
            Ok(_) => Ok(()),
        }
    }
}
//     pub async fn init_jet_stream(&mut self) {
//         self.jet_stream = JetStream::new(self.nats_stream._get()).await
//     }
//     pub async fn init_kv_store(&mut self, bucket_name: &str) {
//         if self.jet_stream.get().is_none() {
//             let err = NatsDriverError::ContextNotExists(
//                 "context not found, creating new one".to_string(),
//             );
//             warn!("{:?}", err);
//             self.init_jet_stream().await;
//         };
//         self.kv_store = NatsKVStorage::new(&self.jet_stream, bucket_name).await
//     }
//
//     pub fn get_client(&self) -> &NatsStream {
//         &self.nats_stream
//     }
//     pub fn _get_context(&self) -> &JetStream {
//         &self.jet_stream
//     }
//     pub fn _get_store(&self) -> &NatsKVStorage {
//         &self.kv_store
//     }
//     pub fn jetstream(&self) -> &Context {
//         self.jet_stream.get().unwrap()
//     }
//     pub async fn kv(&mut self, bucket_name: &str) -> &Store {
//         if self.kv_store.get().is_err() {
//             self.init_kv_store(bucket_name).await;
//         };
//         self.kv_store.get().unwrap()
//     }
//     pub async fn create_stream<S>(&self, config: S) -> Result<Stream, NatsDriverError>
//     where
//         Config: From<S>,
//     {
//         match self.jetstream().create_stream(config).await {
//             Err(err) => Err(NatsDriverError::CreateStreamError(err)),
//             Ok(s) => Ok(s),
//         }
//     }
//     pub async fn get_stream<T: AsRef<str>>(&self, stream: T) -> Result<Stream, NatsDriverError> {
//         match self.jetstream().get_stream(stream).await {
//             Err(err) => Err(NatsDriverError::GetStreamError(err)),
//             Ok(s) => Ok(s),
//         }
//     }
//     pub async fn get_subscriber(&mut self, subject: &str) -> Result<Subscriber, NatsDriverError> {
//         match self.nats_stream.subscriber(subject).await {
//             Ok(s) => Ok(s),
//             Err(err) => Err(NatsDriverError::ErrorSubscribing(err)),
//         }
//     }
//     pub async fn publish(
//         &mut self,
//         subject: &str,
//         message: Bytes,
//     ) -> Result<PublishAckFuture, NatsDriverError> {
//         match self.jet_stream.get() {
//             None => Err(NatsDriverError::ContextNotExists(
//                 "Context doesnt exits".to_owned(),
//             )),
//             Some(c) => match c.publish(subject.to_owned(), message).await {
//                 Ok(ack) => Ok(ack),
//                 Err(err) => Err(NatsDriverError::FailedPublishingToJetStream(err)),
//             },
//         }
//     }
//     pub async fn pull_kv(&self, key: &str) -> Result<Option<Bytes>, NatsDriverError> {
//         match self.kv_store.pull_kv(key).await {
//             Ok(msg) => Ok(msg),
//             Err(err) => Err(NatsDriverError::StorePullKVError(err)),
//         }
//     }
//     pub async fn push_kv(&mut self, key: &str, value: Bytes) -> Result<u64, NatsDriverError> {
//         match self.kv_store.push_kv(key, value).await {
//             Ok(i) => Ok(i),
//             Err(err) => Err(NatsDriverError::StoreError(err)),
//         }
//     }
//     pub async fn update_kv(&mut self, key: &str, value: Bytes) -> Result<u64, NatsDriverError> {
//         match self.kv_store.update_kv(key, value).await {
//             Ok(i) => Ok(i),
//             Err(err) => Err(NatsDriverError::StoreError(err)),
//         }
//     }
//     pub async fn delete_kv(&mut self, key: &str) -> Result<(), NatsDriverError> {
//         match self.kv_store.delete_kv(key).await {
//             Ok(_) => Ok(()),
//             Err(err) => Err(NatsDriverError::StoreError(err)),
//         }
//     }
// }
