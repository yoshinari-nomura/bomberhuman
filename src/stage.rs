use crate::actors::block::*;
use crate::geometry::*;

/// Stage

pub struct Stage {
    pub blocks: Vec<Block>,
    pub width: usize,
    pub height: usize,
}

/// Base information to creat stage
///
/// ```text
///  0 1 2 3 4 5 6 7 8 9 A B C D E
/// 0■■■■■■■■■■■■■■■
/// 1■××□□□□□□□□□××■
/// 2■×■□■□■□■□■□■×■
/// 3■□□□□□□□□□□□□□■
/// 4■□■□■□■□■□■□■□■
/// 5■□□□□□□□□□□□□□■
/// 6■□■□■□■□■□■□■□■
/// 7■□□□□□□□□□□□□□■
/// 8■□■□■□■□■□■□■□■
/// 9■□□□□□□□□□□□□□■
/// A■×■□■□■□■□■□■×■
/// B■××□□□□□□□□□××■
/// C■■■■■■■■■■■■■■■
/// ```
const BLOCK_MAP: [u8; 195] = [
    // 1  2  3  4  5  6  7  8  9  A  B  C  D  E
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0
    1, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 1, // 1
    1, 2, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 2, 1, // 2
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, // 3
    1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, // 4
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, // 5
    1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, // 6
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, // 7
    1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, // 8
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, // 9
    1, 2, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 2, 1, // A
    1, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 1, // B
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // C
];

impl Stage {
    pub fn new() -> Stage {
        let mut blocks: Vec<Block> = vec![];
        for (i, info) in BLOCK_MAP.iter().enumerate() {
            let (x, y) = (((i % 15) as i32) * GS, ((i / 15) as i32) * GS);
            match info {
                1 => blocks.push(Block::hard(x, y)),
                2 => (),
                _ => {
                    if rand::random() {
                        blocks.push(Block::soft(x, y));
                    }
                }
            }
        }
        println!("{}", BLOCK_MAP[0]);

        Stage {
            width: 15,
            height: 13,
            blocks,
        }
    }
}

impl Default for Stage {
    fn default() -> Self {
        Stage::new()
    }
}
