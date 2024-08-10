use std::{sync::Arc, thread};

use log::info;
use omniverlay_core::{
    errors::OmniverlayResult,
    extensions::{
        config::{
            ConfigCategory, ConfigCategoryBuilder, ConfigEnum, ConfigValue, ConfigValueType,
            ExtensionConfigBuilder,
        },
        Extension, ExtensionGeometry, ExtensionInfo, ExtensionLayout, ExtensionState,
    },
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
            .add_category(
                ConfigCategoryBuilder::new("General".to_string())
                    .add_value(
                        "first_name".to_string(),
                        ConfigValue::new(
                            "The first name of the person".to_string(),
                            ConfigValueType::String("Jhon".to_string()),
                        ),
                    )
                    .add_value(
                        "last_name".to_string(),
                        ConfigValue::new(
                            "The last name of the person".to_string(),
                            ConfigValueType::String("Doe".to_string()),
                        ),
                    )
                    .add_value(
                        "age".to_string(),
                        ConfigValue::new(
                            "The age of the person".to_string(),
                            ConfigValueType::Int(30),
                        ),
                    )
                    .add_value(
                        "active".to_string(),
                        ConfigValue::new(
                            "Is the person active".to_string(),
                            ConfigValueType::Bool(true),
                        ),
                    )
                    .build(),
            )
            .add_category(
                ConfigCategoryBuilder::new("Job".to_string())
                    .add_value(
                        "Grade".to_string(),
                        ConfigValue::new(
                            "The grade of the person".to_string(),
                            ConfigValueType::Enum(ConfigEnum::new(
                                "grades".to_string(),
                                "A".to_string(),
                                vec![
                                    "A".to_string(),
                                    "B".to_string(),
                                    "C".to_string(),
                                    "D".to_string(),
                                    "E".to_string(),
                                ],
                            )),
                        ),
                    )
                    .build(),
            )
            .build();

        Self {
            info: Arc::new(Mutex::new(ExtensionInfo {
                name: "Performance".to_string(),
                state: ExtensionState {
                    is_enabled: true,
                    config: Some(config),
                },
                layout: Some(ExtensionLayout {
                    width: 200,
                    height: 200,
                    x: 0,
                    y: 0,
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
