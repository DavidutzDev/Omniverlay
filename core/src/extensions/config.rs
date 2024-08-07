use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    collections::{HashMap, HashSet},
    fs::{create_dir_all, File},
    io::{Read, Write},
    path::PathBuf, sync::{Arc, Mutex},
};

use crate::errors::{OmniverlayError, OmniverlayResult}; // Ensure serde_json is used for JSON serialization

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigEnum {
    pub name: String,
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConfigValue {
    String(String),
    Float(f64),
    Int(i64),
    Bool(bool),
    List(Vec<ConfigValue>),
    Enum(ConfigEnum),
    Path(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigCategory {
    pub name: String,
    pub values: HashMap<String, ConfigValue>,
}

impl ConfigCategory {
    pub fn get_value(&self, name: &str) -> Option<&ConfigValue> {
        self.values.get(name)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionConfig {
    pub categories: Vec<ConfigCategory>,
    pub values: HashMap<String, ConfigValue>, // General configs
}

impl ExtensionConfig {
    pub fn get_value(&self, name: &str) -> Option<&ConfigValue> {
        self.values.get(name)
    }

    pub fn get_category(&self, name: &str) -> Option<&ConfigCategory> {
        self.categories.iter().find(|c| c.name == name)
    }

    pub fn to_json(&self) -> OmniverlayResult<String> {
        let json = serde_json::to_string_pretty(self)?;

        Ok(json)
    }

    pub fn from_json(json: &str) -> OmniverlayResult<Self> {
        let config = serde_json::from_str(json)?;

        Ok(config)
    }

    // Method to match the structure of two ExtensionConfig instances
    pub fn match_structure(&self, other: &Self) -> bool {
        // Compare general values
        if self.values.keys().collect::<HashSet<_>>() != other.values.keys().collect::<HashSet<_>>()
        {
            return false;
        }

        // Check the types of general values
        for (key, value) in &self.values {
            if let Some(other_value) = other.values.get(key) {
                if !Self::value_type_matches(value, other_value) {
                    return false;
                }
            } else {
                return false;
            }
        }

        // Compare categories
        let self_categories: HashMap<_, _> = self
            .categories
            .iter()
            .map(|c| (c.name.clone(), c))
            .collect();
        let other_categories: HashMap<_, _> = other
            .categories
            .iter()
            .map(|c| (c.name.clone(), c))
            .collect();

        if self_categories.keys().collect::<HashSet<_>>()
            != other_categories.keys().collect::<HashSet<_>>()
        {
            return false;
        }

        // Compare category structures
        for (name, self_category) in &self_categories {
            if let Some(other_category) = other_categories.get(name) {
                if !Self::category_structure_matches(self_category, other_category) {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    // Helper method to compare value types
    fn value_type_matches(value1: &ConfigValue, value2: &ConfigValue) -> bool {
        match (value1, value2) {
            (ConfigValue::String(_), ConfigValue::String(_)) => true,
            (ConfigValue::Float(_), ConfigValue::Float(_)) => true,
            (ConfigValue::Int(_), ConfigValue::Int(_)) => true,
            (ConfigValue::Bool(_), ConfigValue::Bool(_)) => true,
            (ConfigValue::List(list1), ConfigValue::List(list2)) => {
                list1.len() == list2.len()
                    && list1
                        .iter()
                        .zip(list2.iter())
                        .all(|(v1, v2)| Self::value_type_matches(v1, v2))
            }
            (ConfigValue::Enum(enum1), ConfigValue::Enum(enum2)) => {
                enum1.name == enum2.name && enum1.values == enum2.values
            }
            (ConfigValue::Path(_), ConfigValue::Path(_)) => true,
            _ => false,
        }
    }

    // Helper method to compare category structures
    fn category_structure_matches(cat1: &ConfigCategory, cat2: &ConfigCategory) -> bool {
        if cat1.values.keys().collect::<HashSet<_>>() != cat2.values.keys().collect::<HashSet<_>>()
        {
            return false;
        }

        for (key, value) in &cat1.values {
            if let Some(other_value) = cat2.values.get(key) {
                if !Self::value_type_matches(value, other_value) {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}

pub struct ExtensionConfigBuilder {
    pub categories: Vec<ConfigCategory>,
    pub values: HashMap<String, ConfigValue>,
}

impl ExtensionConfigBuilder {
    pub fn new() -> Self {
        Self {
            categories: Vec::new(),
            values: HashMap::new(),
        }
    }

    pub fn add_category(mut self, category: ConfigCategory) -> Self {
        self.categories.push(category);
        self
    }

    pub fn add_value(mut self, name: String, value: ConfigValue) -> Self {
        self.values.insert(name, value);
        self
    }

    pub fn build(self) -> ExtensionConfig {
        ExtensionConfig {
            categories: self.categories,
            values: self.values,
        }
    }
}

pub struct ExtensionConfigManager {
    configs: HashMap<String, Arc<Mutex<ExtensionConfig>>>,
}

impl ExtensionConfigManager {
    pub fn new() -> Self {
        Self {
            configs: HashMap::new(),
        }
    }

    pub fn register_config(
        &mut self,
        name: String,
        config: ExtensionConfig,
    ) -> OmniverlayResult<()> {
        self.configs.insert(name, Arc::new(Mutex::new(config)));
        Ok(())
    }

    pub fn get_config(&self, name: &str) -> OmniverlayResult<Arc<Mutex<ExtensionConfig>>> {
        self.configs
            .get(name)
            .cloned()
            .ok_or_else(|| OmniverlayError::ConfigNotFound(name.to_string()))
    }

    // pub fn save_config(&self, name: &str) -> OmniverlayResult<()> {
    //     // if let Some(parent) = self.path.parent() {
    //     //     println!("Parent: {:?}", parent);
    //     //     if !parent.exists() {
    //     //         create_dir_all(parent)?; // Create directory if it doesn't exist
    //     //     }
    //     // }

    //     // let arc_config = self.get_config(name)?;
    //     // let config = arc_config.lock().map_err(|e| OmniverlayError::LockError(e.to_string()))?;

    //     // // Save the configuration to a file
    //     // let file_path = self.path.join(format!("{}.json", name));
    //     // let mut file = File::create(&file_path)?;
    //     // let config_json = config.to_json()?;
    //     // file.write_all(config_json.as_bytes())?;

    //     // Ok(())
    // }

    // pub fn load_config(&self, name: &str) -> OmniverlayResult<ExtensionConfig> {
    //     // let file_path = self.path.join(format!("{}.json", name));

    //     // if !file_path.exists() {
    //     //     return Err(OmniverlayError::ConfigNotFound(name.to_string()));
    //     // }

    //     // let mut file = File::open(&file_path)?;
    //     // let mut contents = String::new();
    //     // file.read_to_string(&mut contents)?;

    //     // let config: ExtensionConfig = ExtensionConfig::from_json(&contents)?;

    //     // Ok(config)
    // }
}
