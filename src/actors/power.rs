use crate::actors::*;
use crate::game_state::*;
use crate::geometry::*;
use crate::*;
use rand::seq::SliceRandom;
// use rand::thread_rng;

/// Power

pub struct Power {
    /// Id to distinguish each Actor
    actor_id: ActorId,
    /// Current status or action of the bomb
    action: u32,
    /// Time to Live
    ttl: i32,
    /// Current location of Power
    pub pnt: Point,
}

impl Power {
    pub fn new(x: i32, y: i32, actor_id: ActorId) -> Self {
        Power {
            actor_id,
            action: 0,
            ttl: 15,
            pnt: grd!(x, y),
        }
    }

    /// Create Powerup Item Randomly
    pub fn random_item(x: i32, y: i32) -> Self {
        let items = [ActorId::BombUp, ActorId::BombPowerUp, ActorId::SpeedUp];
        let actor_id = *items.choose(&mut rand::thread_rng()).unwrap();
        Power::new(x, y, actor_id)
    }

    pub fn alive(&self) -> bool {
        self.ttl > 0
    }

    pub fn draw(&self) {
        screen_put_sprite(self.pnt.x, self.pnt.y, self.actor_id, self.action)
    }

    pub fn update(&mut self, _delta: i32, gs: &GameState) {
        for p in &mut *gs.players_mut() {
            if p.pnt.align_to_grid() == self.pnt {
                p.push_item(self.actor_id);
                self.ttl = 0;
            }
        }

        let fire_exists = gs.fires().iter().any(|f| f.pnt == self.pnt);
        let block_exists = gs.blocks().iter().any(|b| b.pnt == self.pnt);
        if fire_exists && !block_exists {
            self.ttl = 0;
        }
        self.action = (15 - self.ttl) as u32;
    }
}
