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
            action: 0,
            ttl: 1,
            pnt: grd!(x, y),
            bomb_power: 2,
            max_num_bombs: 1,
            speed: 1,
        }
    }

    /// Draw player on screen
    pub fn draw(&self) {
        screen_put_sprite(self.pnt.x, self.pnt.y, self.actor_id, self.action)
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
        let speed = delta / 6 * self.speed as i32; // 1frame = 16ms, speed should be 5 or so.
        let mut action = 0;
        let mut dx = 0;
        let mut dy = 0;

        let mut bombs = gs.bombs_mut();
        let blocks = gs.blocks();

        if key_state.left {
            dx = -speed;
            action += 1;
        }
        if key_state.right {
            dx = speed;
            action += 2;
        }
        if key_state.up {
            dy = -speed;
            action += 4;
        }
        if key_state.down {
            dy = speed;
            action += 8;
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
        self.action = action;

        let (mut dx, mut dy) = self.align_vector_to_grid(dx, dy);
        let xy = pnt!(self.pnt.x + dx, self.pnt.y + dy);

        // Can not move if block exist
        let block_exists = blocks.iter().any(|b| b.pnt.collides_with(xy));
        if block_exists {
            // XXX: Making (dx, dy) zero is bad idea. Should make (dx, dy) be shorten
            // to keep the safe distance, instead.
            dx = 0;
            dy = 0;
        }

        // Can not move if bomb exists
        // However, if the current position overlaps the bomb, it can.
        let bomb_exists = bombs
            .iter()
            .any(|b| !self.pnt.collides_with(b.pnt) && xy.collides_with(b.pnt));
        if bomb_exists {
            // XXX: Making (dx, dy) zero is bad idea. Should make (dx, dy) be shorten
            // to keep the safe distance, instead.
            dx = 0;
            dy = 0;
        }
        self.pnt.x += dx;
        self.pnt.y += dy;

        let fire_exists = gs.fires().iter().any(|f| self.pnt.collides_with(f.pnt));
        if fire_exists {
            self.action = 15;
            self.ttl = -60 * 8;
        }
    }

    /// Correct the direction vector to go through the nearest grid center
    ///
    /// # Algorism
    ///
    /// 1. Let (dx, dy) be the original vector and (gx, gy) be the
    ///    vector from its current position (x, y) to the nearest grid.
    /// 2. Let θ be the angle formed by (dx, dy) and (gx, gy)
    /// 3. if θ is within ±90 degrees (inner product is 0 or greater) → (gx, gy).
    /// 4. else, use (dx, dy)
    ///
    fn align_vector_to_grid(&self, dx: i32, dy: i32) -> (i32, i32) {
        let (cx, cy) = (
            (self.pnt.x + (GS / 2)) / GS * GS,
            (self.pnt.y + (GS / 2)) / GS * GS,
        );
        let (mut gx, mut gy) = (cx - self.pnt.x, cy - self.pnt.y);
        let ip = gx * dx + gy * dy;
        let speed = if dx.abs() > dy.abs() {
            dx.abs()
        } else {
            dy.abs()
        };

        // It's not moving or already on the grid
        if (dx == 0 && dy == 0) || (gx == 0 && gy == 0) {
            return (dx, dy);
        }

        if ip >= 0 {
            // Within 90 degree angle to the center of grid,
            // make it go to the center of the grid.

            // Clip the size of the vector to less than the original speed.
            if gx.abs() > speed {
                gx = gx.signum() * speed;
            }
            if gy.abs() > speed {
                gy = gy.signum() * speed;
            }
            (gx, gy)
        } else {
            // It's moving away from the center of grid.
            (dx, dy)
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
