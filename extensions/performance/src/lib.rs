use std::thread;

use log::info;
use omniverlay_core::{
    errors::OmniverlayResult,
    extensions::{Extension, ExtensionGeometry},
    TAURI_APP_HANDLE,
};
use serde::Serialize;
use sysinfo::System;
use tauri::Manager;

pub struct PerformanceExtension {
    is_enabled: bool,
    geometry: ExtensionGeometry,
    system: System,
}

impl PerformanceExtension {
    pub fn new() -> Self {
        Self {
            is_enabled: false,
            geometry: ExtensionGeometry {
                x: 0,
                y: 0,
                width: 500,
                height: 50,
            },
            system: System::new(),
        }
    }
}

#[derive(Clone, Serialize)]
struct PerformancePayload {
    cpu_usage: f32,
}

impl Extension for PerformanceExtension {
    fn name(&self) -> &'static str {
        "Performance"
    }

    fn geometry(&self) -> OmniverlayResult<&ExtensionGeometry> {
        Ok(&self.geometry)
    }

    fn set_geometry(&mut self, geometry: ExtensionGeometry) -> OmniverlayResult<()> {
        self.geometry = geometry;

        info!("Geometry updated: {:?}", self.geometry);

        Ok(())
    }

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

        self.is_enabled = true;
        Ok(())
    }

    fn is_enabled(&self) -> OmniverlayResult<bool> {
        Ok(self.is_enabled)
    }

    fn disable(&mut self) -> OmniverlayResult<()> {
        self.is_enabled = false;
        Ok(())
    }
}
