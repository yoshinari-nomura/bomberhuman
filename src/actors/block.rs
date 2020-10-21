use crate::actors::*;

/// Block

pub struct Block {
    actor_id: ActorId,
    action: u32,
    pub x: i32,
    pub y: i32,
}

impl Block {
    pub fn new(x: i32, y: i32) -> Self {
        Block {
            actor_id: ActorId::Block,
            action: 0,
            x,
            y,
        }
    }

    pub fn alive(&self) -> bool {
        true
    }

    pub fn draw(&self) {
        screen_put_sprite(self.x, self.y, self.actor_id, self.action)
    }

    pub fn update(&mut self, _delta: i32) {}
}
