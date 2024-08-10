use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};

use crate::errors::{OmniverlayError, OmniverlayResult}; // Ensure serde_json is used for JSON serialization

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigEnum {
    pub name: String,
    pub current: String,
    pub values: Vec<String>,
}

impl ConfigEnum {
    pub fn new(name: String, current: String, values: Vec<String>) -> Self {
        Self { name, current, values }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigValueType {
    String(String),
    Float(f64),
    Int(i64),
    Bool(bool),
    List(Vec<ConfigValueType>),
    Enum(ConfigEnum),
    Path(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigValue {
    pub description: String,
    pub value: ConfigValueType,
}

impl ConfigValue {
    pub fn new(description: String, value: ConfigValueType) -> Self {
        Self { description, value }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigCategory {
    pub name: String,
    pub values: HashMap<String, ConfigValue>,
}

impl ConfigCategory {
    pub fn new(name: String) -> Self {
        Self { name, values: HashMap::new() }
    }

    pub fn add_value(&mut self, name: String, value: ConfigValue) {
        self.values.insert(name, value);
    }

    pub fn get_value(&self, name: &str) -> Option<&ConfigValue> {
        self.values.get(name)
    }
}

pub struct ConfigCategoryBuilder {
    name: String,
    values: HashMap<String, ConfigValue>,
}

impl ConfigCategoryBuilder {
    pub fn new(name: String) -> Self {
        Self {
            name,
            values: HashMap::new(),
        }
    }

    pub fn add_value(mut self, name: String, value: ConfigValue) -> Self {
        self.values.insert(name, value);
        self
    }

    pub fn build(self) -> ConfigCategory {
        ConfigCategory {
            name: self.name,
            values: self.values,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionConfig {
    pub categories: Vec<ConfigCategory>,
}

impl ExtensionConfig {
    pub fn get_value(&self, name: &str) -> Option<(&ConfigCategory, &ConfigValue)> {
        for category in &self.categories {
            if let Some(value) = category.get_value(name) {
                return Some((category, value));
            }
        }
        None
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

    // Helper method to compare category structures
    fn category_structure_matches(cat1: &ConfigCategory, cat2: &ConfigCategory) -> bool {
        if cat1.values.keys().collect::<HashSet<_>>() != cat2.values.keys().collect::<HashSet<_>>()
        {
            return false;
        }

        for (key, value) in &cat1.values {
            if let Some(other_value) = cat2.values.get(key) {
                if !Self::value_type_matches(&value.value, &other_value.value) {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    // Helper method to compare value types
    fn value_type_matches(value1: &ConfigValueType, value2: &ConfigValueType) -> bool {
        match (value1, value2) {
            (ConfigValueType::String(_), ConfigValueType::String(_)) => true,
            (ConfigValueType::Float(_), ConfigValueType::Float(_)) => true,
            (ConfigValueType::Int(_), ConfigValueType::Int(_)) => true,
            (ConfigValueType::Bool(_), ConfigValueType::Bool(_)) => true,
            (ConfigValueType::List(list1), ConfigValueType::List(list2)) => {
                list1.len() == list2.len()
                    && list1
                        .iter()
                        .zip(list2.iter())
                        .all(|(v1, v2)| Self::value_type_matches(v1, v2))
            }
            (ConfigValueType::Enum(enum1), ConfigValueType::Enum(enum2)) => {
                enum1.name == enum2.name && enum1.values == enum2.values
            }
            (ConfigValueType::Path(_), ConfigValueType::Path(_)) => true,
            _ => false,
        }
    }
}

pub struct ExtensionConfigBuilder {
    pub categories: Vec<ConfigCategory>,
}

impl ExtensionConfigBuilder {
    pub fn new() -> Self {
        Self {
            categories: Vec::new(),
        }
    }

    pub fn add_category(mut self, category: ConfigCategory) -> Self {
        self.categories.push(category);
        self
    }

    pub fn build(self) -> ExtensionConfig {
        ExtensionConfig {
            categories: self.categories,
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

    // Other methods related to saving and loading configs can be implemented here
}
