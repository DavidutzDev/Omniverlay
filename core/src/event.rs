use std::fmt::Display;

use once_cell::sync::OnceCell;
use tokio::sync::RwLock;


#[derive(Debug)]
pub enum OmniverlayEventType {
    UpdateExtensions
}

#[derive(Debug)]
pub struct Event {
    pub event_type: OmniverlayEventType,
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type EventHandler = Box<dyn Fn(Event) + Send + Sync>;

pub static EVENT_HANDLER: OnceCell<RwLock<Option<EventHandler>>> = OnceCell::new();

#[macro_export]
macro_rules! invoke_event {
    ($event_type:expr) => {
        {
            use $crate::event::{Event, EVENT_HANDLER};

            // Log the event type
            log::info!("Invoking event: {:?}", $event_type);

            // Create the Event
            let event = Event { event_type: $event_type };

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