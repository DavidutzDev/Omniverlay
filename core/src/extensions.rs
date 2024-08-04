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
    is_enabled: bool,
    geometry: ExtensionGeometry,
}

pub trait Extension: Send {
    fn name(&self) -> &str;
    fn geometry(&self) -> OmniverlayResult<&ExtensionGeometry>;
    fn set_geometry(&mut self, geometry: ExtensionGeometry) -> OmniverlayResult<()>;
    fn enable(&mut self) -> OmniverlayResult<()>;
    fn is_enabled(&self) -> OmniverlayResult<bool>;
    fn disable(&mut self) -> OmniverlayResult<()>;
}

pub struct ExtensionManager {
    extensions: Vec<Box<dyn Extension>>,
}

impl ExtensionManager {
    pub fn new() -> Self {
        Self {
            extensions: Vec::new(),
        }
    }

    pub fn register_extension(&mut self, extension: Box<dyn Extension>) {
        self.extensions.push(extension);
    }

    pub fn enable_extension(&mut self, extension_name: &str) -> OmniverlayResult<()> {
        if let Some(extension) = self
            .extensions
            .iter_mut()
            .find(|e| e.name() == extension_name)
        {
            extension.enable()
        } else {
            Err(OmniverlayError::ExtensionNotFound(
                extension_name.to_string(),
            ))
        }
    }

    pub fn disable_extension(&mut self, extension_name: &str) -> OmniverlayResult<()> {
        if let Some(extension) = self
            .extensions
            .iter_mut()
            .find(|e| e.name() == extension_name)
        {
            extension.disable()
        } else {
            Err(OmniverlayError::ExtensionNotFound(
                extension_name.to_string(),
            ))
        }
    }

    pub fn update_extension(&mut self, info: ExtensionInfo) -> OmniverlayResult<()> {
        if let Some(extension) = self
            .extensions
            .iter_mut()
            .find(|e| e.name() == info.name)
        {

            extension.set_geometry(info.geometry)?;
            
            Ok(())
        } else {
            Err(OmniverlayError::ExtensionNotFound(
                info.name.to_string(),
            ))
        }
    }

    pub fn list_extensions(&self) -> OmniverlayResult<Vec<ExtensionInfo>> {
        let mut extension_infos = Vec::new();

        for extension in &self.extensions {
            extension_infos.push(ExtensionInfo {
                name: extension.name().to_string(),
                is_enabled: extension.is_enabled()?,
                geometry: extension.geometry()?.clone(),
            });
        }

        Ok(extension_infos)
    }
    
}
