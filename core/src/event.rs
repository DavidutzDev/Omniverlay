use std::fmt::Display;

use once_cell::sync::OnceCell;
use tokio::sync::RwLock;


#[derive(Debug)]
pub enum OmniverlayEventType {
    UpdateExtensionData,
}

#[derive(Debug)]
pub struct OmniverlayEvent {
    pub event_type: OmniverlayEventType,
}

impl Display for OmniverlayEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type OmniverlayEventHandler = Box<dyn Fn(OmniverlayEvent) + Send + Sync>;

pub static EVENT_HANDLER: OnceCell<RwLock<Option<OmniverlayEventHandler>>> = OnceCell::new();

#[macro_export]
macro_rules! invoke_event {
    ($event_type:expr) => {
        {
            use $crate::event::{OmniverlayEvent, EVENT_HANDLER};

            // Log the event type
            log::info!("Invoking event: {:?}", $event_type);

            // Create the Event
            let event = OmniverlayEvent { event_type: $event_type };

            // Get the event handler from the OnceCell
            if let Some(handler) = EVENT_HANDLER.get() {
                let handler = handler.read().await;
                if let Some(ref h) = *handler {
                    h(event);
                } else {
                    log::warn!("No event handler set. Event not dispatched.");
                }
            } else {
                log::warn!("Event handler not initialized.");
            }
        }
    };
}