use web_sys::WebGlRenderingContext as GL;
use rand::prelude::*;

use crate::programs::Rectangle;

pub struct GoL {
    dimensions: (u32, u32),
    tiles: Vec<bool>,
    renderer: Rectangle,
}

impl GoL {
    pub fn new(gl: &GL, width: u32, height: u32) -> Self {
        let mut tiles = Vec::<bool>::new();
        let mut rng = rand::thread_rng();

        for _ in 0..width*height {
            tiles.push(rng.gen::<f32>() > 0.8);
        }

        Self {
            dimensions: (width, height),
            tiles,
            renderer: Rectangle::new(&gl),
        }
    }

    fn decode(&self, index: usize) -> (u32, u32) {
       (index as u32 % self.dimensions.0, index as u32 / self.dimensions.0)
    }

    fn encode(&self, x: u32, y: u32) -> usize {
        // Perform a modulo on the length of the tiles vector to loop coordinate space.
        (y * self.dimensions.0 + x) as usize % self.tiles.len() as usize
    }

    pub fn update() {
    }

    pub fn render(&self, gl: &GL, aspect: f32) {
        for (index, &active) in self.tiles.iter().enumerate() {
            let (col, row) = self.decode(index);
            let width = 4.0 / self.dimensions.0 as f32 / aspect;
            let height = 4.0 / self.dimensions.1 as f32;
            let x: f32 = width * col as f32 - 2.0;
            let y: f32 = height * row as f32 - 2.0;
            let color: [f32; 4] = if active { [1.0,1.0,1.0,1.0] } else { [0.0,0.0,0.0,0.0] };
            self.renderer.render(gl, x, y, width, height, color);
            // if active {
            //     crate::log(&format!("{}, {} | {},{} - {},{}", col, row, x, y, width, height));
            // }
        }
    }
}
