use wasm_bindgen::prelude::*;
use web_sys::{OffscreenCanvas, WebGlRenderingContext};

use crate::simulations::{FallingSand, Flock, GoL, Simulation};
// use crate::simulations::GoL;

type GL = web_sys::WebGlRenderingContext;

mod common_funcs;
mod gl_setup;
mod quadtree;
mod rendering;
mod shaders;
mod simulations;
mod utils;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub struct FolioClient {
    gl: WebGlRenderingContext,
    sim: FallingSand,
    canvas: OffscreenCanvas,
}

#[wasm_bindgen]
impl FolioClient {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: OffscreenCanvas) -> Self {
        console_error_panic_hook::set_once();
        let gl = gl_setup::init_webgl_ctx(&canvas).unwrap();
        // let gol = GoL::new(&gl, canvas.width() / 10, canvas.height() / 10);
        //*****let flock = Flock::new(&gl, canvas.width() / 10, canvas.height() / 10);
        let falling_sand = FallingSand::new(&gl, canvas.width() / 7, canvas.height() / 7);

        Self {
            gl,
            canvas,
            sim: falling_sand,
        }
    }

    pub fn update(&mut self, _time: f32, _height: f32, _width: f32) -> Result<(), JsValue> {
        //self.sim.update();
        Ok(())
    }

    pub fn render(&self) {
        self.gl.viewport(
            0,
            0,
            self.gl.drawing_buffer_width(),
            self.gl.drawing_buffer_height(),
        );
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        self.sim.render(&self.gl);

        crate::log("RENDER");
    }
}
