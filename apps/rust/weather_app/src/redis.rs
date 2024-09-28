use redis::{Client, Commands};

pub struct RedisHandler {
    client: Client,
}

impl RedisHandler {
    pub fn new(address: &str) -> Self {
        Self {
            client: redis::Client::open(address).unwrap(),
        }
    }
    pub fn get_keys(&mut self) -> Vec<String> {
        self.client.keys("*").unwrap_or_else(|err| panic!("{err}"))
    }
    pub fn get(&mut self, key: &str) -> Option<Vec<u8>> {
        self.client.get(key).unwrap_or_else(|err| {
            log::warn!("{}\nKey :: {}", err, key);
            self.del(key);
            None
        })
    }
    pub fn del(&mut self, key: &str) {
        let result = self.client.del::<&str, i32>(key).unwrap();
        if result == 1 {
            log::info!("Key: {} - removed successfully", key);
            return;
        }
        log::warn!("Failed remove key: {}", key)
    }
    pub fn post(&mut self, key: &str, value: &[u8]) {
        self.client.set::<&str, &[u8], ()>(key, value).unwrap()
    }
}
