#[macro_use]
pub mod utils;

pub mod actors;
pub mod game_state;
#[macro_use]
pub mod geometry;
pub mod keyboard;
pub mod screen;
pub mod stage;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Main for WASM

#[wasm_bindgen(start)]
pub fn initialize() {
    utils::set_panic_hook();
}
