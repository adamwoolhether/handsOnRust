use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize; // usize will return an integer of bit size matching your CPU's architecture.

// Our game map is represented by a series of tiles, which are limited to a set of
// tile-types, making enums a good fit here.
#[derive(Copy, Clone, PartialEq)]
// derive will add the defined functions to the type.
pub enum TileType {
    Wall,
    Floor,
}

// Map will contain a vector of tiles.
pub struct Map {
    pub tiles: Vec<TileType>,
}

// map_idx enables map indexing. We use `row-first` encoding to store map
// indices.
pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = map_idx(x, y);
                match self.tiles[idx] {
                    TileType::Floor => {
                        ctx.set(x, y, YELLOW, BLACK, to_cp437('.'));
                    }
                    TileType::Wall => {
                        ctx.set(x, y, GREEN, BLACK, to_cp437('#'));
                    }
                }
            }
        }
    }
}
