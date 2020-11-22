use std::collections::HashMap;

use tokio::{sync::mpsc::Sender, time::Duration};

pub struct Cache<T> {
    data: HashMap<String, T>,
    max_size: usize,
    mutator: Sender<CacheMutation>,
    config: CacheConfig,
}

pub struct CacheConfig {
    expiration_time: Duration,
}

impl CacheConfig {
    pub fn new(expiration_time: Duration) -> Self {
        CacheConfig { expiration_time }
    }

    pub fn get_expiration_time(&self) -> &Duration {
        &self.expiration_time
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        CacheConfig::new(Duration::from_secs(30))
    }
}

impl<T> Cache<T> {
    pub fn new(max_size: usize, mutator: Sender<CacheMutation>) -> Self {
        Cache {
            data: HashMap::with_capacity(max_size),
            max_size,
            mutator,
            config: CacheConfig::default(),
        }
    }

    pub fn insert(&mut self, key: String, data: T) {
        self.data.insert(key.clone(), data);

        let duration = self.config.get_expiration_time().clone();
        let mutator = self.mutator.clone();

        tokio::spawn(async move {
            tokio::time::sleep(duration).await;

            if let Err(e) = mutator.send(CacheMutation::Drop(key)).await {
                println!("Failed to send drop operation - {:}", e);
            }
        });
    }

    pub fn get(&self, key: &str) -> Option<&T> {
        self.data.get(key)
    }

    pub fn get_max_size(&self) -> usize {
        self.max_size
    }

    pub fn delete(&mut self, key: String) {
        self.data.remove(&key);
    }
}

pub enum CacheMutation {
    Drop(String),
}
