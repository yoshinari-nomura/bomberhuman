use crate::actors::ActorId;
use crate::geometry::*;
use crate::*;

/// Stage

pub struct Stage {
    map: Vec<u8>,
    width: usize,
    height: usize,
}

// 15x13
// ■■■■■■■■■■■■■■■
// ■□□□□□□□□□□□□□■
// ■□■□■□■□■□■□■□■
// ■□□□□□□□□□□□□□■
// ■□■□■□■□■□■□■□■
// ■□□□□□□□□□□□□□■
// ■□■□■□■□■□■□■□■
// ■□□□□□□□□□□□□□■
// ■□■□■□■□■□■□■□■
// ■□□□□□□□□□□□□□■
// ■□■□■□■□■□■□■□■
// ■□□□□□□□□□□□□□■
// ■■■■■■■■■■■■■■■
impl Stage {
    pub fn new() -> Stage {
        let mut map: Vec<u8> = vec![];
        let blocks: Vec<u16> = vec![
            // LSB is not used
            0b1111111111111110,
            0b1000000000000010,
            0b1010101010101010,
            0b1000000000000010,
            0b1010101010101010,
            0b1000000000000010,
            0b1010101010101010,
            0b1000000000000010,
            0b1010101010101010,
            0b1000000000000010,
            0b1010101010101010,
            0b1000000000000010,
            0b1111111111111110,
        ];
        for y in 0..13 {
            for x in 0..15 {
                if blocks[y] & (0b1000_0000_0000_0000 >> x) != 0 {
                    map.push(1 << (ActorId::Block as u8));
                } else {
                    map.push(0);
                }
            }
        }
        Stage {
            map,
            width: 15,
            height: 13,
        }
    }

    pub fn put(&mut self, gp: Grid, c: ActorId) {
        if self.height == 0 {
            return;
        }
        let offset = gp.y * self.width as i32 + gp.x;
        self.map[offset as usize] |= 1 << c as u8
    }

    pub fn get(&self, gp: Grid, c: ActorId) -> bool {
        let offset = gp.y * self.width as i32 + gp.x;
        self.map[offset as usize] & (1 << c as u8) != 0
    }
    pub fn dump(&self) {
        for y in 0..13 {
            let mut string = String::new();
            for x in 0..15 {
                let c = if self.get(Grid::new(x, y), ActorId::Block) {
                    '■'
                } else {
                    '□'
                };
                string.push(c);
            }
            console_log!("{}", string);
        }
    }
}

impl Default for Stage {
    fn default() -> Self {
        Stage::new()
    }
}
