use crate::interfaces::repo_interface::RepoInterface;
use alert_entity::{AlertEntity, AlertState};
use event_entity::{EventAction, EventEntity, EventType};
use log::{debug, error};

pub struct AlertHandler<'a, R>
where
    R: RepoInterface,
{
    received_alert: AlertEntity,
    repo: &'a mut R,
    service: String,
}

impl<'a, R> AlertHandler<'a, R>
where
    R: RepoInterface,
{
    pub fn init(received_alert: &AlertEntity, repo: &'a mut R) -> Self {
        Self {
            received_alert: received_alert.to_owned(),
            repo,
            service: "occurrence-handler".to_string(),
        }
    }
    pub fn occurrence_handling_flow(&mut self) -> EventEntity {
        //     First step - check if alert with same alert_id exists in storage
        match self.extract_alert_from_repo() {
            None => self.new_occurrence(),
            Some(_) => self.handle_existing_alert(),
        }
    }
    fn new_occurrence(&mut self) -> EventEntity {
        const DROP_DESCRIPTION: &str = "dropping new resolved alert";
        match self.received_alert.get_state() {
            AlertState::Firing => self.first_occurrence(),
            AlertState::Resolved => {
                debug!("{DROP_DESCRIPTION}");
                event_action_drop(
                    &mut self.received_alert,
                    DROP_DESCRIPTION,
                    self.service.as_str(),
                )
            }
        }
    }
    fn handle_existing_alert(&mut self) -> EventEntity {
        const ERROR_MESSAGE: &str = "expected alert to exists in repo but it doesnt";
        match self.extract_alert_from_repo() {
            None => {
                error!("{ERROR_MESSAGE}");
                EventEntity::new(
                    &mut self.received_alert,
                    EventType::Log,
                    EventAction::Failure,
                    ERROR_MESSAGE,
                    self.service.as_str(),
                )
            }
            Some(alert_from_repo) => match alert_from_repo.get_state() {
                AlertState::Firing => self.existing_alert_firing(),
                AlertState::Resolved => self.existing_alert_resolved(),
            },
        }
    }
    fn first_occurrence(&mut self) -> EventEntity {
        const DESCRIPTION: &str = "alert opened";
        let mut new_alert = self.received_alert.clone();
        new_alert.set_new_occurrence_id();
        let new_alert_bytes = new_alert.as_bytes().unwrap();
        self.repo
            .push(new_alert.get_alert_id().to_string(), new_alert_bytes);
        debug!("{DESCRIPTION}");
        EventEntity::new(
            &mut new_alert,
            EventType::Event,
            EventAction::Open,
            DESCRIPTION,
            self.service.as_str(),
        )
    }
    fn existing_alert_firing(&mut self) -> EventEntity {
        const ERROR_DESCRIPTION: &str =
            "old alert has no `occurrence_id` but is expected to have one";
        const REFIRING: &str = "alert refiring";
        match self.received_alert.get_state() {
            AlertState::Firing => {
                let mut old_alert = self.get_existing_alert_from_repo();
                match old_alert.get_occurrence_id() {
                    None => {
                        event_type_log(&mut old_alert, ERROR_DESCRIPTION, self.service.as_str())
                    }
                    Some(id) => {
                        self.received_alert.set_occurrence_id(id.to_string());
                        event_action_drop(&mut self.received_alert, REFIRING, self.service.as_str())
                    }
                }
            }
            AlertState::Resolved => self.resolve(),
        }
    }
    fn existing_alert_resolved(&mut self) -> EventEntity {
        match self.received_alert.get_state() {
            AlertState::Firing => self.reopen(),
            AlertState::Resolved => event_action_drop(
                &mut self.received_alert,
                "alert dropped id 1",
                &self.service,
            ),
        }
    }
    fn reopen(&mut self) -> EventEntity {
        const REOPEN: &str = "alert reopened";
        self.received_alert.set_new_occurrence_id();
        let new_alert = self.received_alert.clone().as_bytes().unwrap();
        self.repo
            .update(self.received_alert.get_alert_id().to_string(), new_alert);
        EventEntity::new(
            &mut self.received_alert,
            EventType::Event,
            EventAction::Reopen,
            REOPEN,
            self.service.as_str(),
        )
    }
    fn resolve(&mut self) -> EventEntity {
        const ERROR_ALERT_NOT_IN_REPO: &str = "failure, alert not found in repo";
        const ERROR_DESERIALIZING: &str = "failed to deserialize alert";
        match self.repo.pull(self.received_alert.get_alert_id()) {
            None => {
                error!("{ERROR_ALERT_NOT_IN_REPO}");
                event_type_log(
                    &mut self.received_alert,
                    ERROR_DESERIALIZING,
                    self.service.as_str(),
                )
            }
            Some(b) => match AlertEntity::from_bytes(b.as_slice()) {
                Ok(new_alert) => self.resolve_alert(new_alert),
                Err(err) => {
                    let err_message = format!("{}: {}", ERROR_DESERIALIZING, err);
                    error!("{}", err_message);
                    event_type_log(
                        &mut self.received_alert,
                        err_message.as_str(),
                        self.service.as_str(),
                    )
                }
            },
        }
    }
    fn resolve_alert(&mut self, mut new_alert: AlertEntity) -> EventEntity {
        const RESOLVED: &str = "alert resolved";

        new_alert.set_state(AlertState::Resolved);
        let alert_bytes = new_alert.as_bytes().unwrap();
        self.repo
            .update(new_alert.get_alert_id().to_string(), alert_bytes);
        debug!("resolved");
        EventEntity::new(
            &mut new_alert,
            EventType::Event,
            EventAction::Resolve,
            RESOLVED,
            self.service.as_str(),
        )
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
    fn get_existing_alert_from_repo(&self) -> AlertEntity {
        match self.repo.pull(self.received_alert.get_alert_id()) {
            Some(alert) => AlertEntity::from_bytes(alert).unwrap(),
            _ => panic!("something really wrong"),
        }
    }
}

fn event_action_drop(alert: &mut AlertEntity, description: &str, name: &str) -> EventEntity {
    EventEntity::new(
        alert,
        EventType::Event,
        EventAction::Drop,
        description,
        name,
    )
}

fn event_type_log(alert: &mut AlertEntity, description: &str, name: &str) -> EventEntity {
    EventEntity::new(
        alert,
        EventType::Log,
        EventAction::Failure,
        description,
        name,
    )
}
