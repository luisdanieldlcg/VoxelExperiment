use apecs::{CanFetch, Write};
use common::{events::Events, state::SysResult};
use vek::Vec2;

#[derive(Debug, Clone, Copy)]
pub enum GameInput {
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    Jump,
    Sneak,
    ToggleWireframe,
}

/// Input struct that holds the state of the keyboard and mouse.
pub struct Input {
    // TODO: consider tracking key releases, as false here just means the key is no longer pressed.
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

type KeyCode = winit::keyboard::KeyCode;

impl Input {
    pub fn press(&mut self, input: KeyCode) {
        self.keys[input as usize] = true;
    }

    pub fn pressed(&self, input: KeyCode) -> bool {
        self.keys[input as usize]
    }

    pub fn release(&mut self, input: KeyCode) {
        self.keys[input as usize] = false;
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

    pub fn cursor_delta(&self) -> Vec2<f32> {
        self.cursor_delta
    }
}

use apecs::*;

use crate::window::WindowEvent;

#[derive(CanFetch)]
pub struct GameInputSystem {
    events: Write<Events<WindowEvent>>,
    input: Write<Input>,
}

pub fn game_input_system(mut system: GameInputSystem) -> SysResult {
    const INPUT_MAPPING: [(KeyCode, GameInput); 7] = [
        (KeyCode::KeyW, GameInput::MoveForward),
        (KeyCode::KeyS, GameInput::MoveBackward),
        (KeyCode::KeyA, GameInput::MoveLeft),
        (KeyCode::KeyD, GameInput::MoveRight),
        (KeyCode::Space, GameInput::Jump),
        (KeyCode::ShiftLeft, GameInput::Sneak),
        (KeyCode::F12, GameInput::ToggleWireframe),
    ];

    for (key, input) in INPUT_MAPPING.iter() {
        if system.input.pressed(*key) {
            system.events.send(WindowEvent::KeyPress(*input, true));
        }
        // TODO: send key releases as well.
    }

    ok()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_input_update() {}
}
