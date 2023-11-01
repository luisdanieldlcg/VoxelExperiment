use vek::Vec2;

use crate::input::GameInput;

/// Represents the various window events that are relevant for the game.
#[derive(Debug, Clone, Copy)]
pub enum WindowEvent {
    /// The window has been requested to close.
    Close,
    /// The window has been resized.
    Resize(Vec2<u32>),
    /// The cursor has been moved.
    CursorMove(Vec2<f32>),
    /// A game key has been pressed or released.
    Input(GameInput, bool),
}

pub struct Events<T> {
    pub events: Vec<T>,
}

impl<T> Events<T> {
    pub fn send(&mut self, event: T) {
        self.events.push(event);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.events.pop()
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}

impl<T> Default for Events<T> {
    fn default() -> Self {
        Self { events: Vec::new() }
    }
}
