use alert_entity::AlertEntity;

pub trait RepoInterface {
    fn pull(&self, key: &str) -> Option<AlertEntity>;
    fn push(&mut self, key: String, value: &mut AlertEntity);
    fn update(&mut self, key: String, value: &AlertEntity);
    fn delete(&mut self, key: &str) -> Option<AlertEntity>;
}
