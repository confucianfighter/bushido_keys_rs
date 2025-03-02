use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct KeyMappingEntry {
    pub key: String,
    #[serde(default)]
    pub modifiers: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ModeConfig {
    pub name: String,
    pub activation_keys: Vec<String>,
    pub key_mapping: HashMap<String, KeyMappingEntry>,
}

#[derive(Debug, Deserialize)]
pub struct ModesConfig {
    pub modes: Vec<ModeConfig>,
}

impl ModeConfig {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_activation_keys(&self) -> &Vec<String> {
        &self.activation_keys
    }

    pub fn get_key_mapping(&self) -> &std::collections::HashMap<String, KeyMappingEntry> {
        &self.key_mapping
    }
}
