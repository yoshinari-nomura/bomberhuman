use wasm_bindgen::prelude::*;

/// Keyboard

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Key {
    Button1 = 0,
    Left = 1,
    Right = 2,
    Up = 4,
    Down = 8,
}

pub struct KeyState {
    pub button1: bool,
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
}

impl KeyState {
    pub fn new() -> Self {
        KeyState {
            button1: false,
            left: false,
            right: false,
            up: false,
            down: false,
        }
    }
}

impl Default for KeyState {
    fn default() -> Self {
        KeyState::new()
    }
}
