use web_sys::WebGlRenderingContext as GL;

use crate::programs::Rectangle;

pub struct GoL {
    dimensions: (u32, u32),
    tiles: Vec<bool>,
    renderer: Rectangle,
}

impl GoL {
    pub fn new(gl: &GL, width: u32, height: u32) -> Self {
        let mut tiles = Vec::<bool>::new();

        for _ in 0..width*height {
            tiles.push(false);
        }

        tiles[0] = true;
        tiles[99] = true;

        Self {
            dimensions: (width, height),
            tiles,
            renderer: Rectangle::new(&gl),
        }
    }

    pub fn update() {
    }

    pub fn render(&self, gl: &GL, aspect: f32) {
        for (index, &active) in self.tiles.iter().enumerate() {
            let col = index as u32 % self.dimensions.0;
            let row = index as u32 / self.dimensions.0;
            let width = 2.0 / self.dimensions.0 as f32 / aspect;
            let height = 2.0 / self.dimensions.1 as f32;
            let x: f32 = width * col as f32 - 1.0;
            let y: f32 = height * row as f32 - 1.0;
            let color: [f32; 4] = if active { [1.0,1.0,1.0,1.0] } else { [0.0,0.0,0.0,0.0] };
            self.renderer.render(gl, x, y, width, height, color);
            // if active {
            //     crate::log(&format!("{}, {} | {},{} - {},{}", col, row, x, y, width, height));
            // }
        }
    }
}
