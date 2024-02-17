use alert_entity::{AlertEntity, AlertState};
use log::{debug, error, info};
use repository::InMem;

pub struct AlertHandler<'a> {
    received_alert: AlertEntity,
    repo: &'a mut InMem,
}

impl<'a> AlertHandler<'a> {
    pub fn init(received_alert: &AlertEntity, repo: &'a mut InMem) -> Self {
        Self {
            received_alert: received_alert.to_owned(),
            repo,
        }
    }
    pub fn occurrence_handling_flow(&mut self) {
        //     First step - check if alert with same alert_id exists in storage
        match self.extract_stored_alert() {
            None => self.new_occurrence(),
            Some(_) => self.handle_existing_alert(),
        }
    }
    fn new_occurrence(&mut self) {
        match self.received_alert.get_state() {
            AlertState::Firing => {
                let mut new_alert = self.received_alert.clone();
                new_alert.set_occurrence_id();
                let new_alert_bytes = new_alert.as_bytes().unwrap();
                self.repo
                    .push(new_alert.get_alert_id(), new_alert_bytes.as_slice());
                debug!("New occurrence");
            }
            AlertState::Resolved => {
                debug!("Dropped new occurrence state Resolved");
            } // Drop here
        }
    }
    fn handle_existing_alert(&mut self) {
        match self.extract_stored_alert() {
            None => error!("expected alert to exists in repo but it doesnt"),
            Some(alert_from_repo) => {
                match alert_from_repo.get_state() {
                    AlertState::Firing => {
                        match self.received_alert.get_state() {
                            AlertState::Firing => {} // Refiring
                            AlertState::Resolved => {
                                self.resolve_occurrence(); // Resolved
                            }
                        }
                    }
                    AlertState::Resolved => match self.received_alert.get_state() {
                        AlertState::Firing => self.reopen(),
                        AlertState::Resolved => {} // Dropped here
                    },
                }
            }
        }
    }
    fn reopen(&mut self) {
        self.received_alert.set_occurrence_id();
        let new_alert = self.received_alert.clone().as_bytes().unwrap();
        self.repo
            .update(self.received_alert.get_alert_id(), &new_alert);
        debug!("Reopened")
    }
    fn resolve_occurrence(&mut self) {
        match self.repo.get(self.received_alert.get_alert_id()) {
            None => error!("failure, alert not found in repo"),
            Some(b) => match AlertEntity::from_bytes(b.as_slice()) {
                Ok(mut new_alert) => {
                    new_alert.set_state(AlertState::Resolved);
                    let alert_bytes = new_alert.as_bytes().unwrap();
                    self.repo.update(new_alert.get_alert_id(), &alert_bytes);
                    debug!("resolved")
                }
                Err(err) => {
                    error!("failed to deserialize alert: {}", err)
                }
            },
        }
    }
    fn extract_stored_alert(&self) -> Option<AlertEntity> {
        match self.repo.get(self.received_alert.get_alert_id()) {
            None => None,
            Some(payload) => match AlertEntity::from_bytes(&payload) {
                Err(_) => None,
                Ok(a) => Some(a),
            },
        }
    }
}
