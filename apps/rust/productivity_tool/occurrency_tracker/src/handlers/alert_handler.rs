use crate::interfaces::repo_interface::RepoInterface;
use alert_entity::{AlertEntity, AlertState};
use log::{debug, error};
use repository::{InMem, InMemoryStorage};

pub struct AlertHandler<'a, R>
where
    R: RepoInterface,
{
    received_alert: AlertEntity,
    repo: &'a mut R,
}

impl<'a, R> AlertHandler<'a, R>
where
    R: RepoInterface,
{
    pub fn init(received_alert: &AlertEntity, repo: &'a mut R) -> Self {
        Self {
            received_alert: received_alert.to_owned(),
            repo,
        }
    }
    pub fn occurrence_handling_flow(&mut self) {
        //     First step - check if alert with same alert_id exists in storage
        match self.extract_alert_from_repo() {
            None => self.new_occurrence(),
            Some(_) => self.handle_existing_alert(),
        }
    }
    fn new_occurrence(&mut self) {
        match self.received_alert.get_state() {
            AlertState::Firing => self.first_occurrence(),
            AlertState::Resolved => {
                debug!("Dropped new occurrence state Resolved");
            }
        }
    }
    fn handle_existing_alert(&mut self) {
        match self.extract_alert_from_repo() {
            None => error!("expected alert to exists in repo but it doesnt"),
            Some(alert_from_repo) => match alert_from_repo.get_state() {
                AlertState::Firing => self.existing_alert_firing(),
                AlertState::Resolved => self.existing_alert_resolved(),
            },
        }
    }
    fn first_occurrence(&mut self) {
        let mut new_alert = self.received_alert.clone();
        new_alert.set_occurrence_id();
        let new_alert_bytes = new_alert.as_bytes().unwrap();
        self.repo
            .push(new_alert.get_alert_id().to_string(), new_alert_bytes);
        debug!("First occurrence");
    }
    fn existing_alert_firing(&mut self) {
        match self.received_alert.get_state() {
            AlertState::Firing => {} // Refiring
            AlertState::Resolved => {
                self.resolve_occurrence(); // Resolved
            }
        }
    }
    fn existing_alert_resolved(&mut self) {
        match self.received_alert.get_state() {
            AlertState::Firing => self.reopen(),
            AlertState::Resolved => {} // Dropped here
        }
    }
    fn reopen(&mut self) {
        self.received_alert.set_occurrence_id();
        let new_alert = self.received_alert.clone().as_bytes().unwrap();
        self.repo
            .update(self.received_alert.get_alert_id().to_string(), new_alert);
        debug!("Reopened")
    }
    fn resolve_occurrence(&mut self) {
        match self.repo.pull(self.received_alert.get_alert_id()) {
            None => error!("failure, alert not found in repo"),
            Some(b) => match AlertEntity::from_bytes(b.as_slice()) {
                Ok(mut new_alert) => {
                    new_alert.set_state(AlertState::Resolved);
                    let alert_bytes = new_alert.as_bytes().unwrap();
                    self.repo
                        .update(new_alert.get_alert_id().to_string(), alert_bytes);
                    debug!("resolved");
                    self.repo.update(
                        new_alert.get_alert_id().to_string(),
                        new_alert.as_bytes().unwrap(),
                    );
                }
                Err(err) => {
                    error!("failed to deserialize alert: {}", err)
                }
            },
        }
    }
    fn extract_alert_from_repo(&self) -> Option<AlertEntity> {
        match self.repo.pull(self.received_alert.get_alert_id()) {
            None => None,
            Some(payload) => match AlertEntity::from_bytes(payload) {
                Err(_) => None,
                Ok(a) => Some(a),
            },
        }
    }
}

impl RepoInterface for InMemoryStorage {
    fn pull(&self, key: &str) -> Option<&Vec<u8>> {
        self.search(key)
    }

    fn push(&mut self, key: String, value: Vec<u8>) {
        self.store(key, value)
    }

    fn update(&mut self, key: String, value: Vec<u8>) {
        self.update(key, value)
    }

    fn delete(&mut self, key: &str) -> Option<Vec<u8>> {
        self.remove(key)
    }
}

impl RepoInterface for InMem {
    fn pull(&self, key: &str) -> Option<&Vec<u8>> {
        self.get(key)
    }

    fn push(&mut self, key: String, value: Vec<u8>) {
        self.push(&key, &value)
    }

    fn update(&mut self, key: String, value: Vec<u8>) {
        self.update(&key, &value)
    }

    fn delete(&mut self, key: &str) -> Option<Vec<u8>> {
        self.delete(key)
    }
}
