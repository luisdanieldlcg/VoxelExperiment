use vek::Vec2;

#[derive(Debug, Clone, Copy)]
pub enum WindowEvent {
    Close,
    Resize(Vec2<u32>),
    CursorMove(Vec2<f32>),
    KeyPress(winit::keyboard::KeyCode, bool),
    ButtonPress(winit::event::MouseButton, bool),
}

#[derive(Debug, Clone, Copy)]
pub enum GameInputEvent {
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    Jump,
    Sneak,
    ToggleWireframe,
}

pub struct Events<T> {
    pub events: Vec<T>,
}

impl<T> Events<T> {
    pub fn push(&mut self, event: T) {
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
