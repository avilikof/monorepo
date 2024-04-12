use alert_entity::AlertEntity;

pub trait RepoInterface {
    async fn pull(&self, key: &str) -> Option<AlertEntity>;
    async fn push(&mut self, key: String, value: &mut AlertEntity);
    async fn update(&mut self, key: String, value: &mut AlertEntity);
    async fn delete(&mut self, key: &str) -> Option<AlertEntity>;
}
