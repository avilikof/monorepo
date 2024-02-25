use crate::interfaces::repo_interface::RepoInterface;
use alert_entity::{AlertEntity, AlertState};
use event_entity::{EventEntity, EventType};
use log::{debug, error};

pub struct OccurrenceHandler<'a, R>
where
    R: RepoInterface,
{
    received_alert: AlertEntity,
    repo: &'a mut R,
    service: String,
}

impl<'a, R> OccurrenceHandler<'a, R>
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
                EventEntity::drop(
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
                EventEntity::log(
                    &mut self.received_alert,
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
        self.repo
            .push(new_alert.get_alert_id().to_string(), &mut new_alert);
        debug!("{DESCRIPTION}");
        EventEntity::open(&mut new_alert, DESCRIPTION, self.service.as_str())
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
                        EventEntity::log(&mut old_alert, ERROR_DESCRIPTION, self.service.as_str())
                    }
                    Some(id) => {
                        self.received_alert.set_occurrence_id(id.to_string());
                        EventEntity::drop(&mut self.received_alert, REFIRING, self.service.as_str())
                    }
                }
            }
            AlertState::Resolved => self.resolve(),
        }
    }
    fn existing_alert_resolved(&mut self) -> EventEntity {
        match self.received_alert.get_state() {
            AlertState::Firing => self.reopen(),
            AlertState::Resolved => EventEntity::drop(
                &mut self.received_alert,
                "alert dropped id 1",
                &self.service,
            ),
        }
    }
    fn reopen(&mut self) -> EventEntity {
        const REOPEN: &str = "alert reopened";
        self.received_alert.set_new_occurrence_id();
        self.repo.update(
            self.received_alert.get_alert_id().to_string(),
            self.received_alert.clone(),
        );
        EventEntity::reopen(&mut self.received_alert, REOPEN, self.service.as_str())
    }
    fn resolve(&mut self) -> EventEntity {
        const ERROR_ALERT_NOT_IN_REPO: &str = "failure, alert not found in repo";
        const ERROR_DESERIALIZING: &str = "failed to deserialize alert";
        match self.repo.pull(self.received_alert.get_alert_id()) {
            None => {
                error!("{ERROR_ALERT_NOT_IN_REPO}");
                EventEntity::log(
                    &mut self.received_alert,
                    ERROR_DESERIALIZING,
                    self.service.as_str(),
                )
            }
            Some(alert) => self.resolve_alert(alert.to_owned()),
        }
    }
    fn resolve_alert(&mut self, mut new_alert: AlertEntity) -> EventEntity {
        const RESOLVED: &str = "alert resolved";

        new_alert.set_state(AlertState::Resolved);
        self.repo
            .update(new_alert.get_alert_id().to_string(), new_alert.clone());
        debug!("resolved");
        EventEntity::resolve(&mut new_alert, RESOLVED, self.service.as_str())
    }
    fn extract_alert_from_repo(&self) -> Option<AlertEntity> {
        match self.repo.pull(self.received_alert.get_alert_id()) {
            None => None,
            Some(payload) => Some(payload.to_owned()),
        }
    }
    fn get_existing_alert_from_repo(&self) -> AlertEntity {
        match self.repo.pull(self.received_alert.get_alert_id()) {
            Some(alert) => alert.to_owned(),
            _ => panic!("something really wrong"),
        }
    }
}
