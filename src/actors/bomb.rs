use crate::actors::*;
use crate::game_state::*;

/// Bomb

pub struct Bomb {
    actor_id: ActorId,
    action: u32,
    ttl: i32,
    pub player_id: u32,
    pub x: i32,
    pub y: i32,
}

impl Bomb {
    pub fn new(player_id: u32, x: i32, y: i32) -> Self {
        Bomb {
            actor_id: ActorId::Bomb,
            action: 0,
            ttl: 300,
            /// who put the bomb
            player_id,
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

    pub fn update(&mut self, delta: i32, gs: &GameState) {
        let fire_exists = gs.fires().iter().any(|f| f.x == self.x && f.y == self.y);

        self.ttl = if self.ttl > delta / 10 {
            self.ttl - delta / 10
        } else {
            0
        };
        if fire_exists && self.ttl > 5 {
            self.ttl = 5;
        }
        self.action = (300 - self.ttl) as u32 * 15 / 300;
    }
}