use std::sync::{Arc, Mutex, MutexGuard};

use env_logger::Env;
use extensions::ExtensionManager;
use log::info;
use once_cell::sync::{Lazy, OnceCell};
use tauri::AppHandle;

pub mod errors;
pub mod extensions;
pub mod utils;

pub struct Omniverlay {
    pub extension_manager: ExtensionManager,
}

impl Omniverlay {
    pub fn new() -> Omniverlay {
        env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

        info!("Omniverlay initialized !");

        Omniverlay {
            extension_manager: ExtensionManager::new(),
        }
    }
}

static OMNIVERLAY_INSTANCE: Lazy<Mutex<Omniverlay>> = Lazy::new(|| Mutex::new(Omniverlay::new()));
pub static TAURI_APP_HANDLE: OnceCell<Arc<AppHandle>> = OnceCell::new();

pub fn get_omniverlay() -> MutexGuard<'static, Omniverlay> {
    OMNIVERLAY_INSTANCE.lock().unwrap()
}