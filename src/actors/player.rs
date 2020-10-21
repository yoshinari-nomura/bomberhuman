use crate::actors::bomb::*;
use crate::actors::*;

use crate::game_state::*;
use crate::keyboard::*;

/// Player

pub struct Player {
    actor_id: ActorId,
    pub id: u32,
    action: u32,
    x: i32,
    y: i32,
}

impl Player {
    pub fn new(id: u32, x: i32, y: i32) -> Self {
        Player {
            id,
            actor_id: ActorId::Player,
            action: 0,
            x,
            y,
        }
    }

    pub fn draw(&self) {
        screen_put_sprite(self.x, self.y, self.actor_id, self.action)
    }

    pub fn update(&mut self, delta: i32, gs: &GameState, key_state: &KeyState) {
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
                let (x, y) = ((self.x + 30) / 60 * 60, (self.y + 30) / 60 * 60);
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
            .any(|b| (b.x - x).abs() < 60 && (b.y - y).abs() < 60);
        if block_exists {
            // XXX: Making (dx, dy) zero is bad idea. Should make (dx, dy) be shorten
            // to keep the safe distance, instead.
            dx = 0;
            dy = 0;
        }
        // Can not move if bomb exists
        // However, if the current position overlaps the bomb, it can.
        let bomb_exists = bombs.iter().any(|b| {
            !((b.x - self.x).abs() < 60 && (b.y - self.y).abs() < 60) && // curretn position is not overlapped
                ((b.x - x).abs() < 60 && (b.y - y).abs() < 60) // boms exists at the same grid
        });
        if bomb_exists {
            // XXX: Making (dx, dy) zero is bad idea. Should make (dx, dy) be shorten
            // to keep the safe distance, instead.
            dx = 0;
            dy = 0;
        }
        self.x += dx;
        self.y += dy;
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
        let (cx, cy) = ((self.x + 30) / 60 * 60, (self.y + 30) / 60 * 60);
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
