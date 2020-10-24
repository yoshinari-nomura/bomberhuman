use crate::actors::*;
use crate::game_state::*;
use crate::geometry::*;
use crate::*;

/// Block

pub struct Block {
    actor_id: ActorId,
    action: u32,
    ttl: i32,
    pub pnt: Point,
}

impl Block {
    pub fn new(x: i32, y: i32) -> Self {
        Block::hard(x, y)
    }

    pub fn hard(x: i32, y: i32) -> Self {
        Block::build(x, y, 30, 0)
    }

    pub fn soft(x: i32, y: i32) -> Self {
        Block::build(x, y, 28, 1)
    }

    pub fn is_soft(&self) -> bool {
        self.ttl < 30
    }

    pub fn alive(&self) -> bool {
        self.ttl > 0
    }

    pub fn draw(&self) {
        screen_put_sprite(self.pnt.x, self.pnt.y, self.actor_id, self.action)
    }

    pub fn update(&mut self, _delta: i32, gs: &GameState) {
        // hardblock →nothing to do.
        if !self.is_soft() {
            return;
        }
        // softblock → check if fired
        if self.ttl == 28 {
            let fire_exists = gs.fires().iter().any(|f| f.pnt == self.pnt);
            if fire_exists {
                self.ttl -= 1;
            }
        } else {
            // burning the block
            self.ttl -= 1;
        }
        self.action = (15 - self.ttl / 2) as u32;
    }

    fn build(x: i32, y: i32, ttl: i32, action: u32) -> Self {
        Block {
            actor_id: ActorId::Block,
            action,
            ttl,
            pnt: grd!(x, y),
        }
    }
}
