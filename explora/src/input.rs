use apecs::{CanFetch, Write};
use core::{event::Events, SysResult};
use vek::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct KeyboardInput {
    pub scan_code: u32,
    pub key_code: Option<KeyCode>,
    pub state: bool,
}

#[derive(CanFetch)]
pub struct KeyboardInputSystem {
    events: Write<Events<KeyboardInput>>,
    input: Write<Input>,
}

pub fn keyboard_input_system(mut state: KeyboardInputSystem) -> SysResult {
    state.input.update();
    for event in state.events.events.iter() {
        if let Some(key_code) = event.key_code {
            match event.state {
                true => state.input.press(key_code),
                false => state.input.release(key_code),
            }
        }
    }
    ok()
}

#[derive(Debug, Clone, Copy)]
pub enum GameInput {
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    Jump,
    Sneak,
    ToggleWireframe,
    ToggleCursor,
}

/// Input struct that holds the state of the keyboard and mouse.
pub struct Input {
    pub pressed: [bool; 256],
    pub just_pressed: [bool; 256],
    pub buttons: [bool; 128],
    pub cursor_delta: Vec2<f32>,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            pressed: [false; 256],
            just_pressed: [false; 256],
            buttons: [false; 128],
            cursor_delta: Vec2::zero(),
        }
    }
}

type KeyCode = winit::keyboard::KeyCode;

impl Input {
    pub fn press(&mut self, input: KeyCode) {
        if !self.pressed[input as usize] {
            self.just_pressed[input as usize] = true;
        }
        self.pressed[input as usize] = true;
    }

    pub fn pressed(&self, input: KeyCode) -> bool {
        self.pressed[input as usize]
    }

    pub fn release(&mut self, input: KeyCode) {
        self.pressed[input as usize] = false;
    }

    pub fn just_pressed(&self, input: KeyCode) -> bool {
        self.just_pressed[input as usize]
    }

    pub fn update(&mut self) {
        self.just_pressed = [false; 256];
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

const INPUT_MAPPING: [(KeyCode, GameInput); 8] = [
    (KeyCode::KeyW, GameInput::MoveForward),
    (KeyCode::KeyS, GameInput::MoveBackward),
    (KeyCode::KeyA, GameInput::MoveLeft),
    (KeyCode::KeyD, GameInput::MoveRight),
    (KeyCode::Space, GameInput::Jump),
    (KeyCode::ShiftLeft, GameInput::Sneak),
    (KeyCode::F1, GameInput::ToggleCursor),
    (KeyCode::F2, GameInput::ToggleWireframe),
];

pub fn game_input_system(mut system: GameInputSystem) -> SysResult {
    for (key, input) in INPUT_MAPPING.iter() {
        if system.input.just_pressed(*key) {
            system.events.send(WindowEvent::JustPressed(*input));
        }
        if system.input.pressed(*key) {
            system.events.send(WindowEvent::KeyPress(*input));
        }
    }
    ok()
}
