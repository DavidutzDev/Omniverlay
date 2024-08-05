use std::{collections::HashMap, sync::{Arc, Mutex}};

use serde::{Deserialize, Serialize};

use crate::errors::{OmniverlayError, OmniverlayResult};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ExtensionGeometry {
    pub width: u32,
    pub height: u32,
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionInfo {
    pub name: String,
    pub is_enabled: bool,
    pub geometry: Option<ExtensionGeometry>,
}

pub trait Extension: Send {
    fn enable(&mut self) -> OmniverlayResult<()>;
    fn disable(&mut self) -> OmniverlayResult<()>;

    fn get_extension_info(&self) -> OmniverlayResult<Arc<Mutex<ExtensionInfo>>>;

    // Default implementations
    fn set_geometry(&mut self, geometry: ExtensionGeometry) -> OmniverlayResult<()> {
        self.get_extension_info()?.lock().unwrap().geometry = Some(geometry);

        Ok(())
    }

    fn set_enabled(&mut self, enabled: bool) -> OmniverlayResult<()> {
        let current_state = self.get_extension_info()?.lock().unwrap().is_enabled;

        if current_state == enabled {
            return Ok(()); // No change needed if the state is already as desired
        }

        // Update the state
        if enabled {
            self.enable()?;
        } else {
            self.disable()?;
        }

        // Update the extension info
        self.get_extension_info()?.lock().unwrap().is_enabled = enabled;

        Ok(())
    }
}

pub struct ExtensionManager {
    extensions: HashMap<String, Arc<Mutex<dyn Extension>>>,
}

impl ExtensionManager {
    pub fn new() -> Self {
        Self {
            extensions: HashMap::new(),
        }
    }

    pub fn register_extension<T: 'static + Extension>(&mut self, extension: T) -> OmniverlayResult<()> {
        // Get extension info
        let info = extension.get_extension_info()?;
        let name = info.lock().unwrap().name.clone();

        // Wrap the extension in an Arc<Mutex<T>>
        let arc = Arc::new(Mutex::new(extension));

        // Insert into the HashMap
        self.extensions.insert(name, arc);

        Ok(())
    }

    pub fn enable_extension(&mut self, extension_name: &str) -> OmniverlayResult<()> {
        let extension = self.get_extension_by_name(extension_name)?;
        let mut mutable = extension.lock().unwrap();

        mutable.set_enabled(true)?;

        Ok(())
    }

    pub fn disable_extension(&mut self, extension_name: &str) -> OmniverlayResult<()> {
        let extension = self.get_extension_by_name(extension_name)?;
        let mut mutable = extension.lock().unwrap();

        mutable.set_enabled(false)?;

        Ok(())
    }

    pub fn update_extension(&mut self, info: ExtensionInfo) -> OmniverlayResult<()> {
        let extension = self.get_extension_by_name(&info.name)?;
        let mut mutable = extension.lock().unwrap();

        mutable.set_enabled(info.is_enabled)?;

        match info.geometry {
            Some(geometry) => mutable.set_geometry(geometry)?,
            None => (),
        }

        Ok(())
    }

    pub fn get_extension_by_name(&self, name: &str) -> OmniverlayResult<Arc<Mutex<dyn Extension>>> {
        let extension = self
            .extensions
            .get(name)
            .ok_or_else(|| OmniverlayError::ExtensionNotFound(name.to_string()))?
            .clone(); // Clone the Arc to get ownership

        Ok(extension)
    }

    pub fn list_extensions(&self) -> OmniverlayResult<Vec<ExtensionInfo>> {
        let mut extension_infos = Vec::new();

        for extension in &self.extensions {
            let info = extension.1.lock().unwrap().get_extension_info()?;
            let clone = info.lock().unwrap().clone();

            extension_infos.push(clone);
        }

        Ok(extension_infos)
    }
    
}
