use crate::{
    errors::{OmniverlayError, OmniverlayResult},
    get_omniverlay,
    utils::fs::get_omniverlay_dir,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

use super::{ExtensionLayout, ExtensionState};

#[async_trait]
pub trait OmniverlayData: Serialize + for<'de> Deserialize<'de> {
    fn name(&self) -> OmniverlayResult<String>;
    async fn apply_to_extensions(&self) -> OmniverlayResult<()>;

    async fn on_load(&mut self) -> OmniverlayResult<()> {
        Ok(())
    }

    fn from_json(json: &str) -> OmniverlayResult<Self> {
        let data = serde_json::from_str(json)?;

        Ok(data)
    }

    fn to_json(&self) -> OmniverlayResult<String> {
        let json = serde_json::to_string(&self)?;
        Ok(json)
    }

    fn dir_path() -> OmniverlayResult<PathBuf>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OmniverlayProfile {
    pub name: String,
    pub extensions: HashMap<String, ExtensionState>, // Assuming the value type is String for simplicity
}

#[async_trait]
impl OmniverlayData for OmniverlayProfile {
    fn name(&self) -> OmniverlayResult<String> {
        Ok(self.name.clone())
    }

    async fn apply_to_extensions(&self) -> OmniverlayResult<()> {
        {
            let omniverlay = get_omniverlay();
            let omniverlay_guard = omniverlay.read().await;

            let extension_manager = omniverlay_guard.get_extension_manager().await;
            let mut extension_manager_guard = extension_manager.write().await;

            // Update with new extensions
            for (extension_name, extension_state) in &self.extensions {
                extension_manager_guard
                    .update_extension_state(extension_name, extension_state.clone())
                    .await?;
            }
        }

        Ok(())
    }

    async fn on_load(&mut self) -> OmniverlayResult<()> {
        {
            let omniverlay = get_omniverlay();
            let omniverlay_guard = omniverlay.read().await;

            let extension_manager = omniverlay_guard.get_extension_manager().await;
            let extension_manager_guard = extension_manager.read().await;

            // Update with new extensions
            for extension in extension_manager_guard.list_extensions().await? {
                if !self.extensions.contains_key(&extension.name) {
                    self.extensions
                        .insert(extension.name.clone(), extension.state.clone());
                }
            }
        }

        Ok(())
    }

    fn dir_path() -> OmniverlayResult<PathBuf> {
        Ok(get_omniverlay_dir()?.join("profiles"))
    }
}

impl Default for OmniverlayProfile {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            extensions: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OmniverlayLayout {
    pub name: String,
    pub extensions: HashMap<String, ExtensionLayout>,
}

#[async_trait]
impl OmniverlayData for OmniverlayLayout {
    fn name(&self) -> OmniverlayResult<String> {
        Ok(self.name.clone())
    }

    async fn apply_to_extensions(&self) -> OmniverlayResult<()> {
        {
            let omniverlay = get_omniverlay();
            let omniverlay_guard = omniverlay.read().await;

            let extension_manager = omniverlay_guard.get_extension_manager().await;
            let mut extension_manager_guard = extension_manager.write().await;

            // Update with new extensions
            for (extension_name, extension_layout) in &self.extensions {
                extension_manager_guard
                    .update_extension_layout(extension_name, extension_layout.clone())
                    .await?;
            }
        }

        Ok(())
    }

    async fn on_load(&mut self) -> OmniverlayResult<()> {
        {
            let omniverlay = get_omniverlay();
            let omniverlay_guard = omniverlay.read().await;

            let extension_manager = omniverlay_guard.get_extension_manager().await;
            let extension_manager_guard = extension_manager.read().await;

            // Update with new extensions
            for extension in extension_manager_guard.list_extensions().await? {
                if let Some(layout) = extension.layout.clone() {
                    if !self.extensions.contains_key(&extension.name) {
                        self.extensions.insert(extension.name.clone(), layout.clone());
                    }
    
                    if let Some(store) = self.extensions.get(&extension.name) {
                        let mut guard = store.clone();

                        guard.width = layout.width;
                        guard.height = layout.height;

                        self.extensions.insert(extension.name.clone(), guard);
                    }
                }
            }
        }

        Ok(())
    }

    fn dir_path() -> OmniverlayResult<PathBuf> {
        Ok(get_omniverlay_dir()?.join("layouts"))
    }
}

impl Default for OmniverlayLayout {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            extensions: HashMap::new(),
        }
    }
}

#[async_trait]
pub trait DataLoader<T: OmniverlayData> {
    async fn load_data(name: String) -> OmniverlayResult<Option<T>>;

    fn save_data(data: &T) -> OmniverlayResult<()>;

    fn list_datas() -> OmniverlayResult<Vec<String>>;
}

pub struct ExtensionDataManager<T>
where
    T: OmniverlayData + Send + Sync + 'static,
{
    pub data: Arc<RwLock<T>>,
}

impl<T> ExtensionDataManager<T>
where
    T: OmniverlayData + Send + Sync + Default + 'static,
{
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(T::default())),
        }
    }

    pub async fn switch_data(&self, name: String) -> OmniverlayResult<()> {
        {
            let mut data = self.data.write().await;

            if let Some(loaded_data) = T::load_data(name).await? {
                *data = loaded_data;
            }
        }

        self.save_data().await?;

        Ok(())
    }

    pub async fn save_data(&self) -> OmniverlayResult<()> {
        let data = self.data.read().await;
        T::save_data(&data)?;

        data.apply_to_extensions().await?;

        Ok(())
    }

    pub async fn get_current(&self) -> OmniverlayResult<Arc<RwLock<T>>> {
        Ok(self.data.clone())
    }
}

#[async_trait]
impl<T> DataLoader<T> for T
where
    T: OmniverlayData + Send + Sync + 'static,
{
    async fn load_data(name: String) -> OmniverlayResult<Option<T>> {
        let dir = T::dir_path()?;
        let file_path = dir.join(format!("{}.json", name));

        if !file_path.exists() {
            return Err(OmniverlayError::DataNotFound(name.to_string()));
        }

        let json = std::fs::read_to_string(&file_path)?;
        let mut data = serde_json::from_str::<T>(&json)?;

        data.on_load().await?;

        Ok(Some(data))
    }

    fn save_data(data: &T) -> OmniverlayResult<()> {
        let json = data.to_json()?;
        let dir = T::dir_path()?;
        let file_path = dir.join(format!("{}.json", data.name()?));

        std::fs::create_dir_all(&dir)?;

        std::fs::write(&file_path, json)?;

        Ok(())
    }

    fn list_datas() -> OmniverlayResult<Vec<String>> {
        let dir = T::dir_path()?;
        let mut datas = Vec::new();

        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                datas.push(name.to_string());
            }
        }

        Ok(datas)
    }
}
