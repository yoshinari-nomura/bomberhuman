use crate::actors::*;
use crate::geometry::*;
use crate::*;

/// Block

pub struct Block {
    actor_id: ActorId,
    action: u32,
    pub pnt: Point,
}

impl Block {
    pub fn new(x: i32, y: i32) -> Self {
        Block {
            actor_id: ActorId::Block,
            action: 0,
            pnt: grd!(x, y),
        }
    }

    pub fn alive(&self) -> bool {
        true
    }

    pub fn draw(&self) {
        screen_put_sprite(self.pnt.x, self.pnt.y, self.actor_id, self.action)
    }

    pub fn update(&mut self, _delta: i32) {}
}
