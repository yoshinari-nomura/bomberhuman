use crate::actors::*;
use crate::game_state::*;
use crate::geometry::*;
use crate::*;

/// Bomb

pub struct Bomb {
    /// Id to distinguish each Actor
    actor_id: ActorId,
    /// Current status or action of the bomb
    action: u32,
    /// Time to Live
    ttl: i32,
    /// Id to distinguish who put the bomb
    pub owner_id: u32,
    /// Current location of Bomb
    pub pnt: Point,
    /// Power of Bomb
    pub power: u8,
}

impl Bomb {
    pub fn new(owner_id: u32, x: i32, y: i32, power: u8) -> Self {
        Bomb {
            actor_id: ActorId::Bomb,
            action: 0,
            ttl: 300,
            owner_id,
            pnt: grd!(x, y),
            power,
        }
    }

    pub fn alive(&self) -> bool {
        self.ttl > 0
    }

    pub fn draw(&self) {
        screen_put_sprite(self.pnt.x, self.pnt.y, self.actor_id, self.action)
    }

    pub fn update(&mut self, delta: i32, gs: &GameState) {
        let fire_exists = gs.fires().iter().any(|f| f.pnt == self.pnt);

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
