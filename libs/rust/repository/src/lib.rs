use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct InMem {
    storage: HashMap<String, Vec<u8>>,
}

impl InMem {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }
    pub fn push(&mut self, key: &str, value: &[u8]) {
        self.storage
            .entry(key.to_string())
            .or_insert(value.to_vec());
    }
    pub fn update(&mut self, key: &str, value: &[u8]) {
        self.storage.insert(key.to_string(), value.to_vec());
    }
    pub fn get(&self, key: &str) -> Option<&Vec<u8>> {
        self.storage.get(key)
    }
    pub fn count_elements(&self) -> usize {
        self.storage.len()
    }
    pub fn get_all_keys(&self) -> Vec<String> {
        let mut keys: Vec<String> = vec![];
        for key in self.storage.keys() {
            keys.push(key.to_string());
        }
        keys
    }
    pub fn delete(&mut self, key: &str) -> Option<Vec<u8>> {
        self.storage.remove(key)
    }
}

impl Default for InMem {
    fn default() -> Self {
        Self::new()
    }
}

pub struct InMemoryStorage {
    data: HashMap<String, (Vec<u8>, Instant)>, // Data stored with timestamp
    ttl: Option<Duration>,                     // Optional time-to-live
}

impl InMemoryStorage {
    pub fn new(ttl: Option<Duration>) -> Self {
        Self {
            data: HashMap::new(),
            ttl,
        }
    }

    pub fn store(&mut self, key: String, value: Vec<u8>) {
        self.data.insert(key, (value, Instant::now()));
    }

    pub fn search(&self, key: &str) -> Option<&Vec<u8>> {
        self.data.get(key).map(|(value, _)| value)
    }

    pub fn update(&mut self, key: String, value: Vec<u8>) {
        if let Some(entry) = self.data.get_mut(&key) {
            entry.0 = value; // Update value
            entry.1 = Instant::now(); // Update timestamp
        }
    }

    pub fn remove(&mut self, key: &str) -> Option<Vec<u8>> {
        self.data.remove(key).map(|(value, _)| value)
    }
    pub fn get_count(&self) -> usize {
        self.data.len()
    }
    pub fn get_keys(&self) -> Vec<&String> {
        self.data.keys().collect()
    }
    // Helper for cleaning up old data
    pub fn cleanup(&mut self) {
        if let Some(ttl) = self.ttl {
            let now = Instant::now();
            self.data.retain(|_, (_, timestamp)| now - *timestamp < ttl);
        }
    }
}
