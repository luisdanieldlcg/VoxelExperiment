use vek::Vec2;

use crate::event::GameInputEvent;

/// Input struct that holds the state of the keyboard and mouse.
pub struct Input {
    pub keys: [bool; 256],
    pub buttons: [bool; 128],
    pub cursor_delta: Vec2<f32>,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            keys: [false; 256],
            buttons: [false; 128],
            cursor_delta: Vec2::zero(),
        }
    }
}

impl Input {
    pub fn is_key_down(&self, key: winit::keyboard::KeyCode) -> bool {
        self.keys[key as usize]
    }

    pub fn is_button_down(&self, button: winit::event::MouseButton) -> bool {
        match button {
            winit::event::MouseButton::Left => self.buttons[0],
            winit::event::MouseButton::Right => self.buttons[1],
            winit::event::MouseButton::Middle => self.buttons[2],
            winit::event::MouseButton::Back => self.buttons[3],
            winit::event::MouseButton::Forward => self.buttons[4],
            winit::event::MouseButton::Other(code) => self.buttons[code as usize],
        }
    }

    pub fn map_game_input(key: winit::keyboard::KeyCode) -> Option<GameInputEvent> {
        let input = match key {
            winit::keyboard::KeyCode::KeyW => GameInputEvent::MoveForward,
            winit::keyboard::KeyCode::KeyS => GameInputEvent::MoveBackward,
            winit::keyboard::KeyCode::KeyA => GameInputEvent::MoveLeft,
            winit::keyboard::KeyCode::KeyD => GameInputEvent::MoveRight,
            winit::keyboard::KeyCode::Space => GameInputEvent::Jump,
            winit::keyboard::KeyCode::ShiftLeft => GameInputEvent::Sneak,
            winit::keyboard::KeyCode::KeyF => GameInputEvent::ToggleWireframe,
            _ => return None,
        };
        Some(input)
    }

    pub fn cursor_delta(&self) -> Vec2<f32> {
        self.cursor_delta
    }
}
