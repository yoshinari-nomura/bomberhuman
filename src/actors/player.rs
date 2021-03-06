use crate::actors::bomb::*;
use crate::actors::*;
use std::cmp::min;

use crate::game_state::*;
use crate::geometry::*;
use crate::keyboard::*;
use crate::*;

/// Player

pub struct Player {
    /// Id to distinguish each Actor
    actor_id: ActorId,
    /// Id to distinguish each Player
    pub id: u32,
    /// Current status or action of the player
    action: u32,
    /// Time to Live
    ttl: i32,
    /// Current location of Player
    pub pnt: Point,
    /// Current bomb power of Player
    bomb_power: u8,
    /// Current max number of bombs
    max_num_bombs: u8,
    /// Current speed of player
    speed: u8,
}

impl Player {
    /// Constructor of Player
    ///
    /// `x`, `y` are the initial positions of the player.
    /// `id` is an id number to distinguish each player.
    pub fn new(id: u32, x: i32, y: i32) -> Self {
        let actor_id = match id {
            0 => ActorId::Player1,
            1 => ActorId::Player2,
            2 => ActorId::Player3,
            3 => ActorId::Player4,
            _ => ActorId::Player1,
        };
        Player {
            id,
            actor_id,
            action: 1000,
            ttl: 1,
            pnt: grd!(x, y),
            bomb_power: 2,
            max_num_bombs: 1,
            speed: 1,
        }
    }

    /// Draw player on screen
    pub fn draw(&self) {
        screen_put_sprite(self.pnt.x, self.pnt.y, self.actor_id, self.action / 1000)
    }

    /// Predicate to check the player is alive
    pub fn alive(&self) -> bool {
        self.ttl > 0
    }

    /// Update function for players
    ///
    /// This function is supposed to be called for each frame of the game.
    /// According to the `key_state`, it acts on the GameState (`gs`).
    pub fn update(&mut self, delta: i32, gs: &GameState, key_state: &KeyState) {
        if !self.alive() {
            self.ttl += 1;
            return;
        }

        if self.action == 15 * 1000 {
            self.action = 1000;
        }

        let speed = delta / 6 * self.speed as i32; // 1frame = 16ms, speed should be 5 or so.
        let mut dx = 0;
        let mut dy = 0;

        let mut bombs = gs.bombs_mut();
        let blocks = gs.blocks();

        if key_state.left {
            dx = -speed;
        }
        if key_state.right {
            dx = speed;
        }
        if key_state.up {
            dy = -speed;
        }
        if key_state.down {
            dy = speed;
        }
        if key_state.button1 {
            let sum: i32 = bombs.iter().fold(
                0,
                |acc, b| if b.owner_id == self.id { acc + 1 } else { acc },
            );
            if sum < self.max_num_bombs as i32 {
                let pnt = self.pnt.align_to_grid();
                if !bombs.iter().any(|b| b.pnt == pnt) {
                    bombs.push(Bomb::new(self.id, pnt.x, pnt.y, self.bomb_power));
                }
            }
        }

        let dxy = self.pnt.adjust_vector_to_grid(pnt!(dx, dy));
        let new_xy = self.pnt + dxy;

        let block_exists = blocks.iter().any(|b| b.pnt.collides_with(new_xy));

        let bomb_exists = bombs
            .iter()
            // bomb exists at new_xy, but does not exist at current position
            .any(|b| !self.pnt.collides_with(b.pnt) && new_xy.collides_with(b.pnt));

        if !block_exists && !bomb_exists {
            self.pnt += dxy;
            self.action = self.vector_to_action(dxy);
        } else {
            // XXX: Making (dx, dy) zero is bad idea. Should make (dx, dy) be shorten
            // to keep the safe distance, instead.
        }

        let fire_exists = gs.fires().iter().any(|f| self.pnt.collides_with(f.pnt));
        if fire_exists {
            self.action = 15 * 1000;
            self.ttl = -60 * 8;
        }
    }

    /// Select animation pattern from delta and current action
    ///
    /// We have 12 action-images in sprites.png
    /// No 1, 4, 7, 10 is the base action
    ///
    /// 0 front-right-leg
    /// 1 front-stay
    /// 2 front-left-leg
    /// 3 left-right-leg
    /// 4 left-stay
    /// 5 left-left-leg
    /// 6 right-right-leg
    /// 7 right-stay
    /// 8 right-left-leg
    /// 9 back-right-leg
    /// 10 back-stay
    /// 11 back-left-leg
    ///
    fn vector_to_action(&self, dxy: Vector) -> u32 {
        if let Some(x) = dxy.cardinal_direction() {
            let base_index = match x {
                Direction::S => 0,
                Direction::W => 3000,
                Direction::E => 6000,
                Direction::N => 9000,
            };
            let current_offset = self.action % 3000;
            base_index + ((current_offset + (dxy.length() * 50) as u32) % 3000)
        } else {
            self.action
        }
    }

    /// bomb, bombpower, speed
    pub fn push_item(&mut self, item_type: ActorId) {
        match item_type {
            ActorId::BombUp => self.max_num_bombs = min(self.max_num_bombs + 1, 8),
            ActorId::BombPowerUp => self.bomb_power = min(self.bomb_power + 1, 8),
            ActorId::SpeedUp => self.speed = min(self.speed + 1, 30),
            _ => (),
        }
    }
}
