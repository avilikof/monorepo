use log::error;

use alert_entity::AlertEntity;
use nats_driver::NatsJetStreamClient;
use repository::InMemoryStorage;

use crate::interfaces::repo_interface::RepoInterface;

impl RepoInterface for InMemoryStorage {
    fn pull(&self, key: &str) -> Option<AlertEntity> {
        match self.search(key) {
            None => None,
            Some(bytes) => match AlertEntity::from_bytes(&bytes) {
                Err(_) => None,
                Ok(alert) => Some(alert),
            },
        }
    }

    fn push(&mut self, key: String, value: &mut AlertEntity) {
        if let Ok(value_bytes) = value.as_bytes() {
            self.store(key, value_bytes)
        }
    }

    fn update(&mut self, key: String, value: &mut AlertEntity) {
        if let Ok(value_bytes) = value.as_bytes() {
            self.update(key, value_bytes)
        }
    }

    fn delete(&mut self, key: &str) -> Option<AlertEntity> {
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

impl RepoInterface for NatsJetStreamClient {
    
}
