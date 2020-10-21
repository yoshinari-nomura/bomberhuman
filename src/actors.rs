pub mod block;
pub mod bomb;
pub mod player;

pub use crate::screen::*;
use wasm_bindgen::prelude::*;

/// Character ID for mapping to Sprite
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActorId {
    Player = 0,
    Bomb = 1,
    Block = 2,
}
