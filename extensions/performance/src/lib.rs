use std::{sync::Arc, thread};

use log::info;
use omniverlay_core::{
    errors::OmniverlayResult,
    extensions::{config::{ConfigValue, ExtensionConfigBuilder}, Extension, ExtensionGeometry, ExtensionInfo, ExtensionLayout, ExtensionState},
};
use serde::Serialize;
use sysinfo::System;
use tauri::Manager;
use tokio::sync::Mutex;

pub struct PerformanceExtension {
    info: Arc<Mutex<ExtensionInfo>>,
    system: System,
}

impl PerformanceExtension {
    pub fn new() -> Self {
        let config = ExtensionConfigBuilder::new()
            .add_value("name".to_string(), ConfigValue::String("Jhon".to_string()))
            .add_value("lastname".to_string(), ConfigValue::String("Doe".to_string()))
            .add_value("age".to_string(), ConfigValue::Int(47))
            .add_value("job".to_string(), ConfigValue::String("Police Officer".to_string()))
            .build();


        Self {
            info: Arc::new(Mutex::new(ExtensionInfo {
                name: "Performance".to_string(),
                state: ExtensionState::default(),
                layout: Some(ExtensionLayout {
                    width: 200,
                    height: 200,
                    x: 0,
                    y: 0
                }),
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
        // let app = TAURI_APP_HANDLE.get().unwrap().clone();

        // thread::spawn(move || loop {
        //     app.emit_all(
        //         "Performance://performance_update",
        //         PerformancePayload { cpu_usage },
        //     )
        //     .unwrap();
        //     thread::sleep(std::time::Duration::from_millis(1000));
        // });

        Ok(())
    }

    fn disable(&mut self) -> OmniverlayResult<()> {
        Ok(())
    }

    fn get_extension_info(&self) -> OmniverlayResult<Arc<Mutex<ExtensionInfo>>> {
        Ok(self.info.clone())
    }
}
