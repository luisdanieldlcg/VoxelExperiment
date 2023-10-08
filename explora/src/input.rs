use winit::event::{ScanCode, VirtualKeyCode};

/// Input struct that holds the state of the keyboard and mouse.
///
/// There are currently 163 keys supported by winit.
/// With 256 we have some extra space for future keys.
pub struct Input {
    pub keys: [bool; 256],
    pub buttons: [bool; 256],
    pub cursor: (f32, f32),
}

impl Default for Input {
    fn default() -> Self {
        Self {
            keys: [false; 256],
            buttons: [false; 256],
            cursor: (0.0, 0.0),
        }
    }
}

impl Input {
    pub fn is_key_down(&self, key: VirtualKeyCode) -> bool {
        self.keys[key as usize]
    }

    pub fn is_button_down(&self, button: ScanCode) -> bool {
        self.buttons[button as usize]
    }

    pub fn cursor(&self) -> (f32, f32) {
        self.cursor
    }
}
