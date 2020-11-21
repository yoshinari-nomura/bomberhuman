use crate::actors::*;
use crate::geometry::*;
use crate::*;

/// Fire

pub struct Fire {
    actor_id: ActorId,
    action: u32,
    ttl: i32,
    pub pnt: Point,
}

impl Fire {
    pub fn new(x: i32, y: i32) -> Self {
        Fire {
            actor_id: ActorId::Fire,
            action: 0,
            ttl: 20,
            pnt: pnt!(x, y),
        }
    }

    pub fn alive(&self) -> bool {
        self.ttl > 0
    }

    pub fn draw(&self) {
        screen_put_sprite(self.pnt.x, self.pnt.y, self.actor_id, self.action)
    }

    pub fn update(&mut self, _delta: i32) {
        // XXX: action is 0-5 for the sake of sprites.png should FIX it.
        self.action = (20 - self.ttl as u32) * 15 / 20;
        self.ttl -= 1;
    }
}
