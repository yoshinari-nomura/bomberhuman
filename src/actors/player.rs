use crate::actors::bomb::*;
use crate::actors::*;
use crate::*;

use crate::game_state::*;
use crate::geometry::*;
use crate::keyboard::*;

/// Player

pub struct Player {
    actor_id: ActorId,
    pub id: u32,
    action: u32,
    ttl: i32,
    x: i32,
    y: i32,
}

impl Player {
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
            x,
            y,
        }
    }

    pub fn draw(&self) {
        screen_put_sprite(self.x, self.y, self.actor_id, self.action)
    }

    pub fn alive(&self) -> bool {
        self.ttl > 0
    }

    pub fn update(&mut self, delta: i32, gs: &GameState, key_state: &KeyState) {
        if !self.alive() {
            self.ttl += 1;
            return;
        }
        let speed = delta / 3; // 1frame = 16ms, speed should be 5 or so.
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
                |acc, b| if b.player_id == self.id { acc + 1 } else { acc },
            );
            if sum < 5 {
                let (x, y) = ((self.x + (GS / 2)) / GS * GS, (self.y + (GS / 2)) / GS * GS);
                if !bombs.iter().any(|b| b.x == x && b.y == y) {
                    bombs.push(Bomb::new(self.id, x, y));
                }
            }
        }
        self.action = action;

        let (mut dx, mut dy) = self.align_vector_to_grid(dx, dy);
        let (x, y) = (self.x + dx, self.y + dy);
        // Can not move if block exist
        let block_exists = blocks
            .iter()
            .any(|b| (b.x - x).abs() < GS && (b.y - y).abs() < GS);
        if block_exists {
            // XXX: Making (dx, dy) zero is bad idea. Should make (dx, dy) be shorten
            // to keep the safe distance, instead.
            dx = 0;
            dy = 0;
        }
        // Can not move if bomb exists
        // However, if the current position overlaps the bomb, it can.
        let bomb_exists = bombs.iter().any(|b| {
            !((b.x - self.x).abs() < GS && (b.y - self.y).abs() < GS) && // curretn position is not overlapped
                ((b.x - x).abs() < GS && (b.y - y).abs() < GS) // boms exists at the same grid
        });
        if bomb_exists {
            // XXX: Making (dx, dy) zero is bad idea. Should make (dx, dy) be shorten
            // to keep the safe distance, instead.
            dx = 0;
            dy = 0;
        }
        self.x += dx;
        self.y += dy;

        let fire_exists = gs
            .fires()
            .iter()
            .any(|f| (f.x - self.x).abs() < GS && (f.y - self.y).abs() < GS);
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
        let (cx, cy) = ((self.x + (GS / 2)) / GS * GS, (self.y + (GS / 2)) / GS * GS);
        let (mut gx, mut gy) = (cx - self.x, cy - self.y);
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
}
