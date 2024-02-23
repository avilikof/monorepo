pub trait RepoInterface {
    fn pull(&self, key: &str) -> Option<&Vec<u8>>;
    fn push(&mut self, key: String, value: Vec<u8>);
    fn update(&mut self, key: String, value: Vec<u8>);
    fn delete(&mut self, key: &str) -> Option<Vec<u8>>;
}
