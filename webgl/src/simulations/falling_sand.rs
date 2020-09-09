use std::cell::Cell;
use web_sys::WebGlRenderingContext as GL;
use rand::prelude::*;

use crate::{
    rendering::Rectangle,
    simulations::Simulation,
    utils::{CoordinateEncoder, FlatEncoder},
};

const X_FLAG: u32    = 0xFFF00000;
const Y_FLAG: u32    = 0x000FFF00;
const TILE_FLAG: u32 = 0x000000FF;

struct TileStorage {
    encoder: FlatEncoder,
    pub tiles: Vec<Cell<u32>>,
    tilemap: Vec<Cell<i32>>,
}

impl TileStorage {
    pub fn new(width: u32, height: u32) -> Self {
        TileStorage {
            encoder: FlatEncoder { dimensions: (width, height) },
            tiles: Vec::<Cell<u32>>::new(),
            tilemap: vec![Cell::<i32>::new(-1); (width * height) as usize],
        }
    }

    pub fn get(&self, x: i32, y: i32) -> Option<&Cell<u32>> {
        let mut result = None;
        if let Some(tile_index) = self.get_index(x, y) {
            result = Some(&self.tiles[tile_index as usize]);
        }
        result
    }

    fn get_index(&self, x: i32, y: i32) -> Option<usize> {
        let mut result = None;
        if let Some(map_index) = self.encoder.encode(x, y) {
            let tile_index = self.tilemap[map_index].get();
            result = if tile_index >= 0 { Some(tile_index as usize) } else { None };
        }
        result
    }

    pub fn insert(&mut self, raw_tile: u32) {
        let (x, y, _tile) = from_raw(raw_tile);
        self.tilemap[self.encoder.encode(x as i32, y as i32).unwrap()].replace(self.tiles.len() as i32);
        self.tiles.push(Cell::<u32>::new(raw_tile));
    }

    pub fn swap(&self, old_coords: (u16, u16), new_coords: (i32, i32)) {
        let old_index = self.encoder.encode(old_coords.0 as i32, old_coords.1 as i32).unwrap();
        // Only swap if new location is valid.
        if let Some(new_index) = self.encoder.encode(new_coords.0, new_coords.1) {
            self.tilemap[old_index].swap(&self.tilemap[new_index]);
        }
    }
}

pub struct FallingSand {
    dimensions: (u32, u32),
    tiles: TileStorage,
    renderer: Rectangle,
}

impl FallingSand {
    pub fn new(gl: &GL, width: u32, height: u32) -> Self {
        let mut tiles = TileStorage::new(width, height);
        let mut rng = rand::thread_rng();
        let encoder = FlatEncoder { dimensions: (width, height) };

        for index in 0..width*height {
            if rng.gen::<f32>() > 0.9 {
                let (x, y) = encoder.decode(index as usize);
                let tile = into_raw(x as u16, y as u16, 1);
                tiles.insert(tile);
            }
        }

        Self {
            dimensions: (width, height),
            tiles,
            renderer: Rectangle::new(&gl),
        }
    }
}

impl Simulation for FallingSand {
    fn update(&mut self) {
        for raw_tile in self.tiles.tiles.iter() {
            if raw_tile.get() & TILE_FLAG > 0 {
                let (x, y, tile) = from_raw(raw_tile.get());
                let below = self.tiles.get(x as i32, y as i32 - 1);
                if below == None && y > 0 {
                    self.tiles.swap((x, y), (x as i32, y as i32 - 1));
                    raw_tile.replace(into_raw(x, y - 1, tile));
                }
            }
        }
    }

    fn render(&self, gl: &GL) {
        for tile in &self.tiles.tiles {
            let (x, y, tile) = from_raw(tile.get());
            let width = 2.0 / self.dimensions.0 as f32;
            let height = 2.0 / self.dimensions.1 as f32;
            let x: f32 = width * x as f32 - 1.0;
            let y: f32 = height * y as f32 - 1.0;
            let color: [f32; 4] = if tile == 1 {
                [237.0 / 256.0, 201.0 / 256.0, 175.0 / 256.0, 1.0]
            } else {
                [0.0,0.0,0.0,0.0]
            };
            self.renderer.render(gl, x, y, width, height, color);
        }
    }
}

fn from_raw(data: u32) -> (u16, u16, u8) {
    let x = ((data & X_FLAG) >> 20) as u16;
    let y = ((data & Y_FLAG) >> 8) as u16;
    let tile = (data & TILE_FLAG) as u8;
    (x, y, tile)
}

fn into_raw(x: u16, y: u16, tile: u8) -> u32 {
    ((x as u32) << 20) + ((y as u32) << 8) + tile as u32
}
