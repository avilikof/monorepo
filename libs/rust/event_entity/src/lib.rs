use alert_entity::AlertEntity;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventEntity {
    alert: AlertEntity,
    service: String,
    action: EventAction,
    event_type: EventType,
    description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EventAction {
    Open,
    Reopen,
    Resolve,
    Update,
    Drop,
    Failure,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EventType {
    Log,
    Event,
}

impl EventEntity {
    pub fn new(
        alert: &mut AlertEntity,
        event_type: EventType,
        action: EventAction,
        description: &str,
        service: &str,
    ) -> Self {
        Self {
            alert: alert.to_owned(),
            service: service.to_string(),
            event_type,
            action,
            description: description.to_string(),
        }
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(bytes)
    }
    pub fn as_bytes(&mut self) -> serde_json::Result<Vec<u8>> {
        serde_json::to_vec(&self)
    }
}
