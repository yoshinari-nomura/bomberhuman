mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(module = "/screen.js")]
extern "C" {
    fn screen_put_sprite(x: i32, y: i32, class_id: ClassId, action: u32);
    fn screen_clear_rect(x: i32, y: i32, width: u32, height: u32);
}

/// Character ID for mapping to Sprite

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClassId {
    Player = 0,
}

/// Player

struct Player {
    id: ClassId,
    action: u32,
    x: i32,
    y: i32,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Player {
            id: ClassId::Player,
            action: 0,
            x,
            y,
        }
    }

    pub fn draw(&self) {
        screen_put_sprite(self.x, self.y, self.id, self.action)
    }

    pub fn update(&mut self, delta: i32, key_state: &KeyState) {
        let factor = 2;
        let mut action = 0;

        if key_state.left {
            self.x -= delta / factor;
            action += 1;
        }
        if key_state.right {
            self.x += delta / factor;
            action += 2;
        }
        if key_state.up {
            self.y -= delta / factor;
            action += 4;
        }
        if key_state.down {
            self.y += delta / factor;
            action += 8;
        }
        self.action = action;
    }
}


/// Keyboard

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Key {
    Left = 1,
    Right = 2,
    Up = 4,
    Down = 8,
}

pub struct KeyState {
    left: bool,
    right: bool,
    up: bool,
    down: bool,
}

impl KeyState {
    pub fn new() -> Self {
        KeyState {
            left: false,
            right: false,
            up: false,
            down: false,
        }
    }
}


/// Game State

#[wasm_bindgen]
pub struct GameState {
    pub width: u32,
    pub height: u32,
    key_state: KeyState,
    players: Vec<Player>,
}

#[wasm_bindgen]
impl GameState {
    pub fn new(width: u32, height: u32) -> Self {
        GameState {
            width,
            height,
            key_state: KeyState::new(),
            players: vec![Player::new(450, 380)],
        }
    }

    pub fn update(&mut self, delta: i32) {
        for p in &mut self.players {
            p.update(delta, &self.key_state);
        }
    }

    pub fn draw(&self) {
        screen_clear_rect(0, 0, self.width, self.height);
        for p in &self.players {
            p.draw();
        }
    }

    pub fn toggle_key(&mut self, key: Key, state: bool) {
        match key {
            Key::Left => self.key_state.left = state,
            Key::Right => self.key_state.right = state,
            Key::Up => self.key_state.up = state,
            Key::Down => self.key_state.down = state,
        }
    }
}


/// Main for WASM

#[wasm_bindgen(start)]
pub fn initialize() {
    utils::set_panic_hook();
}
