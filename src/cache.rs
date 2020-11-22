use std::collections::HashMap;

use super::api::Config;

pub struct Cache {
    data: HashMap<String, Config>,
    max_size: usize,
}

impl Cache {
    pub fn new(max_size: usize) -> Self {
        Cache {
            data: HashMap::with_capacity(max_size),
            max_size,
        }
    }

    pub fn insert(&mut self, config: Config) {
        self.data.insert(config.get_name().to_owned(), config);
    }

    pub fn get(&self, name: &str) -> Option<&Config> {
        self.data.get(name)
    }

    pub fn get_max_size(&self) -> usize {
        self.max_size
    }
}

impl Default for Cache {
    fn default() -> Self {
        Cache::new(100)
    }
}
