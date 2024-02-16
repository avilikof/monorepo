use std::collections::HashMap;

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
    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        self.storage.get(key).cloned()
    }
}

impl Default for InMem {
    fn default() -> Self {
        Self::new()
    }
}
