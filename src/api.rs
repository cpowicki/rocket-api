use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    name: String,
    value: u32,
}

impl Config {
    pub fn new(name: String, value: u32) -> Config {
        Config { name, value }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
