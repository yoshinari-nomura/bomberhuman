mod utils;

use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use web_sys::console::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(module = "/src/javascripts/screen.js")]
extern "C" {
    fn screen_put_sprite(x: i32, y: i32, class_id: ClassId, action: u32);
    fn screen_clear_rect(x: i32, y: i32, width: u32, height: u32);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

/// Character ID for mapping to Sprite

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClassId {
    Player = 0,
    Bomb = 1,
}

/// Bomb

struct Bomb {
    class_id: ClassId,
    action: u32,
    ttl: i32,
    x: i32,
    y: i32,
}

impl Bomb {
    pub fn new(x: i32, y: i32) -> Self {
        Bomb {
            class_id: ClassId::Bomb,
            action: 0,
            ttl: 300,
            x,
            y,
        }
    }

    pub fn alive(&self) -> bool {
        if self.ttl > 0 {
            true
        } else {
            false
        }
    }

    pub fn draw(&self) {
        screen_put_sprite(self.x, self.y, self.class_id, self.action)
    }

    pub fn update(&mut self, delta: i32, _key_state: &KeyState) {
        self.ttl = if self.ttl > delta / 10 {
            self.ttl - delta / 10
        } else {
            0
        };
        self.action = (300 - self.ttl) as u32 * 15 / 300;
    }
}

/// Player

struct Player {
    class_id: ClassId,
    id: u32,
    action: u32,
    x: i32,
    y: i32,
}

impl Player {
    pub fn new(id: u32, x: i32, y: i32) -> Self {
        Player {
            id,
            class_id: ClassId::Player,
            action: 0,
            x,
            y,
        }
    }

    pub fn draw(&self) {
        screen_put_sprite(self.x, self.y, self.class_id, self.action)
    }

    pub fn update_mut(&self, delta: i32, gs: &GameState, key_state: &KeyState) {
        if key_state.button1 {
            let mut bombs = gs.bombs.borrow_mut();
            if bombs.len() < 5 {
                bombs.push(Bomb::new(self.x, self.y))
            }
        }
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
    Button1 = 0,
    Left = 1,
    Right = 2,
    Up = 4,
    Down = 8,
}

pub struct KeyState {
    button1: bool,
    left: bool,
    right: bool,
    up: bool,
    down: bool,
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

/// Game State

#[wasm_bindgen]
pub struct GameState {
    pub width: u32,
    pub height: u32,
    key_states: Vec<KeyState>,
    players: Vec<Player>,
    bombs: RefCell<Vec<Bomb>>,
}

#[wasm_bindgen]
impl GameState {
    pub fn new(width: u32, height: u32) -> Self {
        GameState {
            width,
            height,
            key_states: vec![KeyState::new(), KeyState::new()],
            players: vec![Player::new(0, 450, 380), Player::new(1, 0, 0)],
            bombs: RefCell::new(vec![]),
        }
    }

    pub fn update(&mut self, delta: i32) {
        for p in &mut self.players {
            p.update(delta, &self.key_states[p.id as usize]);
        }
        for p in &self.players {
            p.update_mut(delta, &self, &self.key_states[p.id as usize]);
        }
        for b in &mut *self.bombs.borrow_mut() {
            b.update(delta, &self.key_states[0]);
        }
        self.cleanup()
    }

    pub fn draw(&self) {
        screen_clear_rect(0, 0, self.width, self.height);
        for p in &self.players {
            p.draw();
        }
        for b in &*self.bombs.borrow() {
            b.draw();
        }
    }

    pub fn toggle_key(&mut self, bind: u32, key: Key, state: bool) {
        let bind = bind as usize;
        match key {
            Key::Button1 => self.key_states[bind].button1 = state,
            Key::Left => self.key_states[bind].left = state,
            Key::Right => self.key_states[bind].right = state,
            Key::Up => self.key_states[bind].up = state,
            Key::Down => self.key_states[bind].down = state,
        }
    }

    fn cleanup(&mut self) {
        let mut bombs = self.bombs.borrow_mut();
        let mut i = 0;
        while i < bombs.len() {
            if bombs[i].alive() {
                // console_log!("len bomb[{}] is still alive ttl:{}", i, bombs[i].ttl);
                i += 1;
            } else {
                bombs.swap_remove(i);
            }
        }
    }
}

/// Main for WASM

#[wasm_bindgen(start)]
pub fn initialize() {
    utils::set_panic_hook();
}
