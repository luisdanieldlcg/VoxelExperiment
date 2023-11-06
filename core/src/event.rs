use apecs::{ok, Write};

use crate::SysResult;

/// Represents a type that can be stored in an [`Events<E>`] resource
pub trait Event: Send + Sync + 'static {}

impl<T> Event for T where T: Send + Sync + 'static {}

pub struct Events<E: Event> {
    pub events: Vec<E>,
}

impl<T: Event> Events<T> {
    pub fn send(&mut self, event: T) {
        self.events.push(event);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.events.pop()
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }

    // this should be called once per frame
    pub fn update(&mut self) {
        self.events.clear();
    }
}

impl<T: Event> Default for Events<T> {
    fn default() -> Self {
        Self { events: Vec::new() }
    }
}

/// A generic update system for events
pub fn event_update_system<E: Event>(mut events: Write<Events<E>>) -> SysResult {
    events.update();
    ok()
}
