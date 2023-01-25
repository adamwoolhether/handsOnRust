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

    // in_bounds ensures that the player cannot walk off of the map.
    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    // can_enter_tile checks if the player can walk into the desired tile, preventing it
    // from walking into walls.
    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }

    // try_idx determines if a tile's index coordinates are valid and do not fall outside
    // of the maps boundaries.
    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }
}
