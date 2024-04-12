use async_nats::jetstream;
use async_nats::jetstream::kv::{EntryError, Store};
use bytes::Bytes;

use crate::nats_context::NatsContext;

#[derive(Debug)]
pub enum NatsStoreError {
    PullKVError(EntryError),
    StoreNotExists,
    PutError(String),
    DeleteKVError(String),
}
#[derive(Default)]
pub struct NatsStorage {
    storage: Option<Store>,
}
impl NatsStorage {
    pub async fn new(context: &NatsContext, bucket_name: &str) -> Self {
        Self {
            storage: Some(Self::init_storage(context, bucket_name).await),
        }
    }
    async fn init_storage(context: &NatsContext, bucket_name: &str) -> Store {
        match context.get() {
            None => {
                panic!("context doesnt exists")
            }
            Some(c) => match c.get_key_value(bucket_name.to_owned()).await {
                Ok(kv) => kv,
                Err(_) => c
                    .create_key_value(jetstream::kv::Config {
                        bucket: bucket_name.to_owned(),
                        ..Default::default()
                    })
                    .await
                    .unwrap(),
            },
        }
    }
    pub fn get(&self) -> Result<&Store, NatsStoreError> {
        match &self.storage {
            None => Err(NatsStoreError::StoreNotExists),
            Some(s) => Ok(s),
        }
    }
    pub async fn pull_kv(&self, key: &str) -> Result<Option<Bytes>, NatsStoreError> {
        match &self.storage {
            Some(s) => match s.get(key).await {
                Ok(m) => Ok(m),
                Err(err) => Err(NatsStoreError::PullKVError(err)),
            },
            None => Err(NatsStoreError::StoreNotExists),
        }
    }
    pub async fn push_kv(&mut self, key: &str, value: Bytes) -> Result<u64, NatsStoreError> {
        match &self.storage {
            Some(store) => match store.create(key, value).await {
                Ok(i) => Ok(i),
                Err(err) => Err(NatsStoreError::PutError(format!("{}", err))),
            },
            None => todo!()
        }
    }
    pub async fn update_kv(&mut self, key: &str, value: Bytes) -> Result<u64, NatsStoreError> {
        match &self.storage {
            Some(store) => match store.put(key, value).await {
                Ok(i) => Ok(i),
                Err(err) => Err(NatsStoreError::PutError(format!("{}", err))),
            },
            None => todo!(),
        }
    }
    pub async fn delete_kv(&mut self, key: &str) -> Result<(), NatsStoreError> {
        match &self.storage {
            Some(s) => match s.delete(key).await {
                Ok(_) => Ok(()),
                Err(err) => Err(NatsStoreError::DeleteKVError(format!("{err}"))),
            },
            None => Err(NatsStoreError::StoreNotExists),
        }
    }
}
