use std::{sync::{Arc, Mutex}, thread};

use log::info;
use omniverlay_core::{
    errors::OmniverlayResult,
    extensions::{Extension, ExtensionGeometry, ExtensionInfo},
    TAURI_APP_HANDLE,
};
use serde::Serialize;
use sysinfo::System;
use tauri::Manager;

pub struct PerformanceExtension {
    info: Arc<Mutex<ExtensionInfo>>,
    system: System,
}

impl PerformanceExtension {
    pub fn new() -> Self {
        Self {
            info: Arc::new(Mutex::new(ExtensionInfo {
                name: "Performance".to_string(),
                is_enabled: false,
                geometry: None,
            })),
            system: System::new(),
        }
    }
}

#[derive(Clone, Serialize)]
struct PerformancePayload {
    cpu_usage: f32,
}

impl Extension for PerformanceExtension {

    fn enable(&mut self) -> OmniverlayResult<()> {
        self.system.refresh_cpu_all();
        let cpu_usage = self.system.global_cpu_usage();
        let app = TAURI_APP_HANDLE.get().unwrap().clone();

        thread::spawn(move || loop {
            app.emit_all(
                "Performance://performance_update",
                PerformancePayload { cpu_usage },
            )
            .unwrap();
            thread::sleep(std::time::Duration::from_millis(1000));
        });

        Ok(())
    }

    fn disable(&mut self) -> OmniverlayResult<()> {
        Ok(())
    }

    fn get_extension_info(&self) -> OmniverlayResult<Arc<Mutex<ExtensionInfo>>> {
        Ok(self.info.clone())
    }
}
