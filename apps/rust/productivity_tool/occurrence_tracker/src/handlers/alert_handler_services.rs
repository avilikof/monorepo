use bytes::Buf;
use futures_util::future::err;
use log::error;

use alert_entity::AlertEntity;
use nats_driver_v2::NatsDriver;
use repository::InMemoryStorage;

use crate::interfaces::repo_interface::RepoInterface;

impl RepoInterface for InMemoryStorage {
    async fn pull(&self, key: &str) -> Option<AlertEntity> {
        match self.search(key) {
            None => None,
            Some(bytes) => match AlertEntity::from_bytes(&bytes) {
                Err(_) => None,
                Ok(alert) => Some(alert),
            },
        }
    }

    async fn push(&mut self, key: String, value: &mut AlertEntity) {
        if let Ok(value_bytes) = value.as_bytes() {
            self.store(key, value_bytes)
        }
    }

    async fn update(&mut self, key: String, value: &mut AlertEntity) {
        if let Ok(value_bytes) = value.as_bytes() {
            self.update(key, value_bytes)
        }
    }

    async fn delete(&mut self, key: &str) -> Option<AlertEntity> {
        match self.remove(key) {
            None => None,
            Some(value_as_bytes) => match AlertEntity::from_bytes(value_as_bytes.as_slice()) {
                Err(err) => {
                    error!("{err}");
                    None
                }
                Ok(alert) => Some(alert),
            },
        }
    }
}

impl RepoInterface for NatsDriver {
    async fn pull(&self, key: &str) -> Option<AlertEntity> {
        self.
        match self.pull_kv(key).await {
            Ok(msg) => match msg {
                None => None,
                Some(m) => match AlertEntity::from_bytes(m.chunk()) {
                    Err(err) => {
                        error!("{err}");
                        None
                    }
                    Ok(alert) => Some(alert),
                },
            },
            Err(err) => {
                error!("{:?}", err);
                None
            }
        }
    }

    async fn push(&mut self, key: String, value: &mut AlertEntity) {
        let value_as_byte = value.as_bytes().unwrap();
        match self.push_kv(&key, bytes::Bytes::from(value_as_byte)).await {
            Ok(_) => {}
            Err(err) => error!("{:#?}", err),
        }
    }

    async fn update(&mut self, key: String, value: &mut AlertEntity) {
        let value_as_byte = value.as_bytes().unwrap();
        match self.push_kv(&key, bytes::Bytes::from(value_as_byte)).await {
            Ok(_) => {}
            Err(err) => error!("{:#?}", err),
        }
    }

    async fn delete(&mut self, key: &str) -> Option<AlertEntity> {
        match self.delete_kv(key).await {
            Ok(_) => None,
            Err(err) => {
                error!("{:#?}", err);
                None
            }
        }
    }
}
