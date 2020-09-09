use web_sys::WebGlRenderingContext as GL;
use rand::prelude::*;
use rayon::prelude::*;

use crate::{
    rendering::Rectangle,
    simulations::Simulation,
    utils::{CoordinateEncoder, FlatEncoder},
};

pub struct FallingSand {
    dimensions: (u32, u32),
    tiles: Vec<u32>,
    renderer: Rectangle,
    encoder: FlatEncoder,
}

impl FallingSand {
    pub fn new(gl: &GL, width: u32, height: u32) -> Self {
        let mut tiles = Vec::<u32>::new();
        let mut rng = rand::thread_rng();

        for _ in 0..width*height {
            tiles.push((rng.gen::<f32>() > 0.9) as u32);
        }

        Self {
            dimensions: (width, height),
            tiles,
            renderer: Rectangle::new(&gl),
            encoder: FlatEncoder { dimensions: (width, height) },
        }
    }
}

impl Simulation for FallingSand {
    fn update(&mut self) {
        for index in 0..self.dimensions.0*self.dimensions.1 {
            if self.tiles[index as usize] == 1 {
                let (x, y) = self.encoder.decode(index as usize);
                if let Some(below) = self.encoder.encode(x as i32, y as i32 - 1) {
                    if self.tiles[below] == 0 {
                        self.tiles[below] = self.tiles[index as usize];
                        self.tiles[index as usize] = 0;
                    }
                }
            }
        }
    }

    fn render(&self, gl: &GL) {
        for (index, &tile) in self.tiles.iter().enumerate() {
            let (col, row) = self.encoder.decode(index);
            let width = 2.0 / self.dimensions.0 as f32;
            let height = 2.0 / self.dimensions.1 as f32;
            let x: f32 = width * col as f32 - 1.0;
            let y: f32 = height * row as f32 - 1.0;
            let color: [f32; 4] = if tile == 1 {
                [237.0 / 256.0, 201.0 / 256.0, 175.0 / 256.0, 1.0]
            } else {
                [0.0,0.0,0.0,0.0]
            };
            self.renderer.render(gl, x, y, width, height, color);
        }
    }
}
