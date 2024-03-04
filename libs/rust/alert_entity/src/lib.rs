use std::time::{SystemTime, UNIX_EPOCH};

use log::{error, warn};
use rand::distributions::uniform::SampleBorrow;
use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize, Debug, Clone, Eq, PartialEq)]
pub enum AlertState {
    Firing,
    Resolved,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AlertEntity {
    #[serde(rename = "occurrenceId")]
    occurrence_id: Option<String>,
    timestamp: u128,
    description: String,
    state: AlertState,
    #[serde(rename = "alertId")]
    alert_id: String,
}

impl AlertEntity {
    pub fn random() -> AlertEntity {
        AlertEntity {
            occurrence_id: None,
            timestamp: get_time_epoch_nano(),
            description: "random alert".to_string(),
            state: get_random_state(),
            alert_id: get_random_number(1000).to_string(),
        }
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, serde_json::Error> {
        convert_empty_str_to_none(serde_json::from_slice(bytes))
    }
    pub fn as_bytes(&mut self) -> serde_json::Result<Vec<u8>> {
        serde_json::to_vec(&convert_none_to_empty_string(self))
    }
    pub fn get_occurrence_id(&self) -> Option<&str> {
        match self.occurrence_id.as_ref() {
            None => None,
            Some(id) => Some(id.as_str()),
        }
    }
    pub fn get_timestamp(&self) -> &u128 {
        self.timestamp.borrow()
    }
    pub fn get_description(&self) -> &str {
        self.description.as_str()
    }
    pub fn get_state(&self) -> &AlertState {
        &self.state
    }
    pub fn get_alert_id(&self) -> &str {
        self.alert_id.as_str()
    }
    fn convert_occurrence_id(&mut self, id: Option<String>) {
        match &self.occurrence_id {
            Some(o) => {
                if o.is_empty() {
                    self.occurrence_id = id.to_owned();
                } else {
                    warn!("id is set cannot change");
                }
            }
            None => self.occurrence_id = id.to_owned(),
        }
    }
    pub fn set_new_occurrence_id(&mut self) {
        match &self.occurrence_id {
            None => {
                self.occurrence_id = Some(get_random_number(99999999999999999).to_string());
            }
            Some(id) => {
                error!("cannot set ID it already exists: '{}'", id)
            }
        }
    }
    pub fn set_occurrence_id(&mut self, id: String) {
        match &self.occurrence_id {
            None => self.occurrence_id = Some(id),
            Some(id) => {
                error!("cannot set ID it already exists: '{}'", id)
            }
        }
    }
    pub fn set_state(&mut self, state: AlertState) {
        if state == self.get_state().clone() {
            warn!("trying to update alert state with same value")
        } else {
            self.state = state
        }
    }
}

impl<'de> Deserialize<'de> for AlertState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?.to_lowercase();
        match s.as_str() {
            "firing" => Ok(AlertState::Firing),
            "resolved" => Ok(AlertState::Resolved),
            "Firing" => Ok(AlertState::Firing),
            "Resolved" => Ok(AlertState::Resolved),
            _ => Err(serde::de::Error::custom("unknown variant")),
        }
    }
}

fn get_time_epoch_nano() -> u128 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(t) => t.as_nanos(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

fn convert_empty_str_to_none(
    alert: Result<AlertEntity, serde_json::Error>,
) -> Result<AlertEntity, serde_json::Error> {
    match alert {
        Ok(mut a) => match &a.occurrence_id {
            Some(o) => {
                if o.is_empty() {
                    a.convert_occurrence_id(None);
                }
                Ok(a)
            }
            None => Ok(a),
        },
        Err(err) => Err(err),
    }
}

fn convert_none_to_empty_string(alert: &mut AlertEntity) -> AlertEntity {
    match alert.get_occurrence_id() {
        None => {
            alert.convert_occurrence_id(Some("".to_string()));
            alert.to_owned()
        }
        Some(_) => alert.to_owned(),
    }
}

fn get_random_state() -> AlertState {
    let alert_states = vec![AlertState::Resolved, AlertState::Firing];
    let mut rng = rand::thread_rng();
    if let Some(random_state) = alert_states.choose(&mut rng) {
        random_state.to_owned()
    } else {
        error!("could not choose random state");
        panic!("error needs to be handled")
    }
}

fn get_random_number(max_number: u128) -> u128 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=max_number)
}
