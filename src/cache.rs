use std::collections::HashMap;

use tokio::time::Instant;

pub struct Cache<T> {
    data: HashMap<String, CacheEntry<T>>,
    max_size: usize
}

pub struct CacheEntry<T> {
    data : T,
    created: Instant
}

impl <T> CacheEntry<T> {

    pub fn new(data : T) -> CacheEntry<T> {
        CacheEntry {
            data,
            created: Instant::now(),
        }
    }

    pub fn into_inner(&self) -> &T {
        &self.data
    }

    pub fn get_created_time(&self) -> &Instant {
        &self.created
    }
}


impl <T> Cache <T> {
    pub fn new(max_size: usize) -> Self {
        Cache {
            data: HashMap::with_capacity(max_size),
            max_size
        }
    }

    pub fn insert(&mut self, key : String, data: T) {
        let entry = CacheEntry::new(data);
        self.data.insert(key, entry);
    }

    pub fn get(&self, key: &str) -> Option<&T> {
        self.data.get(key).map(CacheEntry::into_inner)
    }

    pub fn get_max_size(&self) -> usize {
        self.max_size
    }
}

impl <T> Default for Cache<T> {
    fn default() -> Self {
        Cache::new(100)
    }
}