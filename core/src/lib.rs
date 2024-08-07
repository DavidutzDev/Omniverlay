use core::panic;
use std::sync::Arc;

use env_logger::Env;
use errors::OmniverlayResult;
use extensions::{data::{DataLoader, ExtensionDataManager, OmniverlayData, OmniverlayLayout, OmniverlayProfile}, ExtensionManager};
use log::info;
use once_cell::sync::Lazy;
use tokio::sync::RwLock;
use utils::fs::get_omniverlay_dir;

pub mod errors;
pub mod event;
pub mod extensions;
pub mod utils;
pub struct Omniverlay {
    extension_manager: Arc<RwLock<ExtensionManager>>,
    profile_manager: Arc<RwLock<ExtensionDataManager<OmniverlayProfile>>>,
    layout_manager: Arc<RwLock<ExtensionDataManager<OmniverlayLayout>>>,
}

impl Omniverlay {
    pub fn new() -> Omniverlay {
        env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

        let _ = get_omniverlay_dir().unwrap_or_else(|_| panic!("Failed to get Omniverlay directory"));

        info!("Omniverlay initialized !");

        Omniverlay {
            extension_manager: Arc::new(RwLock::new(ExtensionManager::new())),
            profile_manager: Arc::new(RwLock::new(ExtensionDataManager::new())),
            layout_manager: Arc::new(RwLock::new(ExtensionDataManager::new())),
        }
    }

    pub async fn startup(&self) -> OmniverlayResult<()> {
        {
            let profile_manager_guard = self.profile_manager.read().await;

            profile_manager_guard.switch_data("default".to_string()).await?;
        }

        {
            let layout_manager_guard = self.layout_manager.read().await;

            layout_manager_guard.switch_data("default".to_string()).await?;
        }

        Ok(())
    }

    pub async fn get_extension_manager(&self) -> Arc<RwLock<ExtensionManager>> {
        self.extension_manager.clone()
    }

    pub async fn get_profile_manager(&self) -> Arc<RwLock<ExtensionDataManager<OmniverlayProfile>>> {
        self.profile_manager.clone()
    }

    pub async fn get_layout_manager(&self) -> Arc<RwLock<ExtensionDataManager<OmniverlayLayout>>> {
        self.layout_manager.clone()
    }

    // pub async fn get_profile_manager(&self) -> Arc<RwLock<ProfileManager>> {
    //     self.profile_manager.clone()
    // }
}

static OMNIVERLAY_INSTANCE: Lazy<Arc<RwLock<Omniverlay>>> = Lazy::new(|| Arc::new(RwLock::new(Omniverlay::new())));

pub fn get_omniverlay() -> Arc<RwLock<Omniverlay>> {
    OMNIVERLAY_INSTANCE.clone()
}