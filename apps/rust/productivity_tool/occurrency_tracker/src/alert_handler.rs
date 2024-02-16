use alert_entity::{AlertEntity, AlertState};
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
    pub fn alert_is_new(&mut self) -> bool {
        match self.extract_stored_alert() {
            None => {
                if *self.received_alert.get_state() == AlertState::Firing {
                    self.repo.push(
                        self.received_alert.get_alert_id(),
                        &self.received_alert.clone().as_bytes().unwrap(),
                    );
                    return true;
                }
            }
            Some(_) => return false,
        }
        false
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
