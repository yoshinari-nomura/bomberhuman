pub mod block;
pub mod bomb;
pub mod fire;
pub mod player;

pub use crate::screen::*;
use wasm_bindgen::prelude::*;

/// Character ID for mapping to Sprite
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActorId {
    Player1 = 0,
    Player2 = 1,
    Player3 = 2,
    Player4 = 3,
    Bomb = 4,
    Block = 5,
    Fire = 6,
}
