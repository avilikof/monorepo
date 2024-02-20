use crate::interfaces::repo_interface::RepoInterface;
use repository::{InMem, InMemoryStorage};

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
