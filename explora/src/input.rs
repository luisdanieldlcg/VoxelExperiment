use apecs::{ok, Write};
use common::SysResult;
use vek::{Vec2, Vec3};

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

pub type Key = winit::keyboard::KeyCode;

impl Input {
    pub fn press(&mut self, input: Key) {
        if !self.pressed[input as usize] {
            self.just_pressed[input as usize] = true;
        }
        self.pressed[input as usize] = true;
    }

    pub const fn move_direction(&self) -> Vec3<f32> {
        vek::Vec3::new(
            (self.pressed(GameInput::MoveRight) as i32 - self.pressed(GameInput::MoveLeft) as i32)
                as f32,
            (self.pressed(GameInput::Jump) as i32 - self.pressed(GameInput::Sneak) as i32) as f32,
            (self.pressed(GameInput::MoveForward) as i32
                - self.pressed(GameInput::MoveBackward) as i32) as f32,
        )
    }

    pub const fn pressed(&self, input: GameInput) -> bool {
        match key_mapping(input) {
            Some(key) => self.pressed[key as usize],
            None => false,
        }
    }

    pub const fn just_pressed(&self, input: GameInput) -> bool {
        match key_mapping(input) {
            Some(key) => self.just_pressed[key as usize],
            None => false,
        }
    }

    pub fn press_mouse(&mut self, button: winit::event::MouseButton) {
        match button {
            winit::event::MouseButton::Left => self.buttons[0] = true,
            winit::event::MouseButton::Right => self.buttons[1] = true,
            winit::event::MouseButton::Middle => self.buttons[2] = true,
            winit::event::MouseButton::Back => self.buttons[3] = true,
            winit::event::MouseButton::Forward => self.buttons[4] = true,
            winit::event::MouseButton::Other(code) => self.buttons[code as usize] = true,
        }
    }

    pub fn release_mouse(&mut self, button: winit::event::MouseButton) {
        match button {
            winit::event::MouseButton::Left => self.buttons[0] = false,
            winit::event::MouseButton::Right => self.buttons[1] = false,
            winit::event::MouseButton::Middle => self.buttons[2] = false,
            winit::event::MouseButton::Back => self.buttons[3] = false,
            winit::event::MouseButton::Forward => self.buttons[4] = false,
            winit::event::MouseButton::Other(code) => self.buttons[code as usize] = false,
        }
    }

    pub fn release(&mut self, input: Key) {
        self.pressed[input as usize] = false;
    }

    pub fn update(&mut self) {
        self.just_pressed = [false; 256];
    }

    pub const fn is_button_down(&self, button: winit::event::MouseButton) -> bool {
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

const fn key_mapping(key: GameInput) -> Option<Key> {
    match key {
        GameInput::MoveForward => Some(Key::KeyW),
        GameInput::MoveBackward => Some(Key::KeyS),
        GameInput::MoveLeft => Some(Key::KeyA),
        GameInput::MoveRight => Some(Key::KeyD),
        GameInput::Jump => Some(Key::Space),
        GameInput::Sneak => Some(Key::ShiftLeft),
        GameInput::ToggleCursor => Some(Key::Period),
        GameInput::ToggleWireframe => Some(Key::F12),
    }
}

pub fn input_system(mut input: Write<Input>) -> SysResult {
    input.update();
    ok()
}
