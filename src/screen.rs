use wasm_bindgen::prelude::*;

use crate::actors::ActorId;

#[wasm_bindgen(module = "/src/javascripts/screen.js")]
extern "C" {
    pub fn screen_put_sprite(x: i32, y: i32, actor_id: ActorId, action: u32);
    pub fn screen_clear_rect(x: i32, y: i32, width: u32, height: u32);
}
