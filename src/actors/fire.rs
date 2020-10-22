use crate::actors::*;

/// Fire

pub struct Fire {
    actor_id: ActorId,
    action: u32,
    ttl: i32,
    pub x: i32,
    pub y: i32,
}

impl Fire {
    pub fn new(x: i32, y: i32) -> Self {
        Fire {
            actor_id: ActorId::Fire,
            action: 0,
            ttl: 20,
            x,
            y,
        }
    }

    pub fn alive(&self) -> bool {
        self.ttl > 0
    }

    pub fn draw(&self) {
        screen_put_sprite(self.x, self.y, self.actor_id, self.action)
    }

    pub fn update(&mut self, _delta: i32) {
        self.ttl -= 1;
    }
}
