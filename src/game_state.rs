use std::cell::{Ref, RefCell, RefMut};
use wasm_bindgen::prelude::*;

use crate::screen::*;
use crate::*;
use actors::block::Block;
use actors::bomb::Bomb;
use actors::fire::Fire;
use actors::player::Player;
use keyboard::*;

/// Game State

#[wasm_bindgen]
pub struct GameState {
    pub width: u32,
    pub height: u32,
    key_states: Vec<KeyState>,
    blocks: RefCell<Vec<Block>>,
    bombs: RefCell<Vec<Bomb>>,
    fires: RefCell<Vec<Fire>>,
    players: RefCell<Vec<Player>>,
}

#[wasm_bindgen]
impl GameState {
    pub fn new(width: u32, height: u32) -> Self {
        let mut blocks = vec![];

        // Put perimeter wall
        for y in 0..13 {
            for x in 0..15 {
                if x == 0 || x == 14 || y == 0 || y == 12 {
                    let block = Block::new(x * 60, y * 60);
                    blocks.push(block);
                }
            }
        }
        for x in &[2, 4, 6, 8, 10, 12] {
            for y in &[2, 4, 6, 8, 10] {
                let block = Block::new(x * 60, y * 60);
                blocks.push(block);
            }
        }

        // Create players
        let mut players = vec![];
        for (i, grid) in [(13, 11), (1, 1), (13, 1), (1, 11)].iter().enumerate() {
            players.push(Player::new(i as u32, grid.0 * 60, grid.1 * 60));
        }

        GameState {
            width,
            height,
            key_states: vec![
                KeyState::new(),
                KeyState::new(),
                KeyState::new(),
                KeyState::new(),
            ],
            players: RefCell::new(players),
            bombs: RefCell::new(vec![]),
            blocks: RefCell::new(blocks),
            fires: RefCell::new(vec![]),
        }
    }

    // delta = ms (1frame = 16.6ms)
    // average delta = 17
    pub fn update(&mut self, delta: i32) {
        let gs = &self;
        for p in &mut *self.players_mut() {
            p.update(delta, gs, &self.key_states[p.id as usize]);
        }
        for b in &mut *self.bombs_mut() {
            b.update(delta, gs)
        }
        for b in &mut *self.blocks_mut() {
            b.update(delta);
        }
        for f in &mut *self.fires_mut() {
            f.update(delta);
        }
        self.cleanup()
    }

    pub fn draw(&self) {
        screen_clear_rect(0, 0, self.width, self.height);
        for p in &*self.players() {
            p.draw();
        }
        for b in &*self.bombs() {
            b.draw();
        }
        for w in &*self.blocks() {
            w.draw();
        }
        for f in &*self.fires() {
            f.draw();
        }
    }

    pub fn toggle_key(&mut self, bind: u32, key: Key, state: bool) {
        let ks = &mut self.key_states[bind as usize];
        match key {
            Key::Button1 => ks.button1 = state,
            Key::Left => ks.left = state,
            Key::Right => ks.right = state,
            Key::Up => ks.up = state,
            Key::Down => ks.down = state,
        }
    }
}

impl GameState {
    pub fn blocks(&self) -> Ref<Vec<Block>> {
        self.blocks.borrow()
    }

    pub fn blocks_mut(&self) -> RefMut<Vec<Block>> {
        self.blocks.borrow_mut()
    }

    pub fn bombs(&self) -> Ref<Vec<Bomb>> {
        self.bombs.borrow()
    }

    pub fn bombs_mut(&self) -> RefMut<Vec<Bomb>> {
        self.bombs.borrow_mut()
    }

    pub fn players(&self) -> Ref<Vec<Player>> {
        self.players.borrow()
    }

    pub fn players_mut(&self) -> RefMut<Vec<Player>> {
        self.players.borrow_mut()
    }

    pub fn fires(&self) -> Ref<Vec<Fire>> {
        self.fires.borrow()
    }

    pub fn fires_mut(&self) -> RefMut<Vec<Fire>> {
        self.fires.borrow_mut()
    }

    fn cleanup(&mut self) {
        let mut bombs = self.bombs_mut();
        let mut i = 0;
        while i < bombs.len() {
            if bombs[i].alive() {
                i += 1;
            } else {
                self.fire(bombs[i].x, bombs[i].y, 5);
                bombs.swap_remove(i);
            }
        }

        let mut fires = self.fires_mut();
        let mut i = 0;
        while i < fires.len() {
            if fires[i].alive() {
                i += 1;
            } else {
                fires.swap_remove(i);
            }
        }
    }

    fn fire(&self, x: i32, y: i32, power: u8) {
        let mut fires = self.fires_mut();
        let (sx, sy) = ((x + 30) / 60 * 60, (y + 30) / 60 * 60);
        let mut p;
        let mut x;
        let mut y;
        for (dx, dy) in &[(0, -60), (0, 60), (-60, 0), (60, 0)] {
            p = power;
            x = sx;
            y = sy;
            loop {
                let block_exists = self.blocks().iter().any(|b| b.x == x && b.y == y);
                if block_exists || p == 0 {
                    break;
                }
                fires.push(Fire::new(x, y));
                p -= 1;
                x += dx;
                y += dy;
            }
        }
    }
}
