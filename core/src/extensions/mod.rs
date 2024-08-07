use std::{collections::HashMap, sync::Arc};
use async_trait::async_trait;
use config::{ExtensionConfig, ExtensionConfigManager};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use crate::errors::{OmniverlayError, OmniverlayResult};

pub mod config;
pub mod data;

// Define Extension structs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionInfo {
    pub name: String,
    pub state: ExtensionState,
    pub layout: Option<ExtensionLayout>,
    //pub geometry: Option<ExtensionGeometry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionState {
    pub is_enabled: bool,
    pub config: Option<ExtensionConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionLayout {
    pub width: u32,
    pub height: u32,
    pub x: u32,
    pub y: u32,
}

impl Default for ExtensionState {
    fn default() -> Self {
        Self {
            is_enabled: false,
            config: None,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ExtensionGeometry {
    pub width: u32,
    pub height: u32,
    pub x: u32,
    pub y: u32,
}

// Define Extension trait with default implementations
#[async_trait]
pub trait Extension: Send {
    fn enable(&mut self) -> OmniverlayResult<()>;
    fn disable(&mut self) -> OmniverlayResult<()>;

    fn get_extension_info(&self) -> OmniverlayResult<Arc<Mutex<ExtensionInfo>>>;

    // Default implementations
    // async fn set_geometry(&mut self, geometry: ExtensionGeometry) -> OmniverlayResult<()> {
    //     self.get_extension_info()?.lock().await.geometry = Some(geometry);
    //     Ok(())
    // }
    async fn update_layout(&mut self, layout: ExtensionLayout) -> OmniverlayResult<()> {
        self.get_extension_info()?.lock().await.layout = Some(layout);
        
        Ok(())
    }

    async fn update_state(&mut self, new_state: ExtensionState) -> OmniverlayResult<()> {
        self.get_extension_info()?.lock().await.state = new_state;

        Ok(())
    }

    async fn set_enabled(&mut self, enabled: bool) -> OmniverlayResult<()> {
        let info = self.get_extension_info()?;
        let mut guard = info.lock().await;

        if guard.state.is_enabled == enabled {
            return Ok(()); // No change needed
        }

        if enabled {
            self.enable()?;
        } else {
            self.disable()?;
        }

        guard.state.is_enabled = enabled;
        Ok(())
    }
}

// Define ExtensionManager to manage extensions
pub struct ExtensionManager {
    config_manager: ExtensionConfigManager,
    extensions: HashMap<String, Arc<Mutex<dyn Extension>>>,
}

impl ExtensionManager {
    pub fn new() -> Self {
        let config_manager = ExtensionConfigManager::new();

        Self {
            config_manager,
            extensions: HashMap::new(),
        }
    }

    pub async fn register_extension<T: 'static + Extension>(&mut self, extension: T) -> OmniverlayResult<()> {
        let info = extension.get_extension_info()?;
        let guard = info.lock().await;

        let name = guard.name.clone();
        
        // if let Some(config) = guard.config.clone() {

        //     // match self.config_manager.load_config(name.as_ref()) {
        //     //     Ok(config) => {
        //     //         self.config_manager.register_config(name.clone(), config)?;
        //     //         info!("Registered config for extension {}", name);
        //     //     },
        //     //     Err(OmniverlayError::ConfigNotFound(_)) => {
        //     //         self.config_manager.register_config(name.clone(), config)?; // Register default config
        //     //         info!("Registered default config for extension {}", name);
        //     //     },
        //     //     Err(e) => {
        //     //         return Err(e);
        //     //     }
        //     // };
        // }

        let arc = Arc::new(Mutex::new(extension));

        self.extensions.insert(name, arc);

        Ok(())
    }

    pub async fn enable_extension(&mut self, extension_name: &str) -> OmniverlayResult<()> {
        let extension = self.get_extension_by_name(extension_name)?;
        let mut guard = extension.lock().await;

        guard.set_enabled(true).await?;

        Ok(())
    }

    pub async fn disable_extension(&mut self, extension_name: &str) -> OmniverlayResult<()> {
        let extension = self.get_extension_by_name(extension_name)?;
        let mut guard = extension.lock().await;

        guard.set_enabled(false).await?;
        
        Ok(())
    }

    pub async fn update_extension_state(&mut self, extension_name: &str, state: ExtensionState) -> OmniverlayResult<()> {
        let extension = self.get_extension_by_name(extension_name)?;
        let mut guard = extension.lock().await;

        guard.update_state(state).await?;

        Ok(())
    }

    pub async fn update_extension_layout(&mut self, extension_name: &str, layout: ExtensionLayout) -> OmniverlayResult<()> {
        let extension = self.get_extension_by_name(extension_name)?;
        let mut guard = extension.lock().await;

        guard.update_layout(layout).await?;

        Ok(())
    }

    pub fn get_extension_by_name(&self, name: &str) -> OmniverlayResult<Arc<Mutex<dyn Extension>>> {
        self.extensions.get(name)
            .cloned()
            .ok_or_else(|| OmniverlayError::ExtensionNotFound(name.to_string()))
    }

    pub async fn list_extensions(&self) -> OmniverlayResult<Vec<ExtensionInfo>> {
        let mut extension_infos = Vec::new();
        for extension in self.extensions.values() {
            let info = extension.lock().await.get_extension_info()?;
            extension_infos.push(info.lock().await.clone());
        }
        Ok(extension_infos)
    }
}
