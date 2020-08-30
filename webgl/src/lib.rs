use wasm_bindgen::prelude::*;
use web_sys::*;

use crate::simulations::GoL;

type GL = web_sys::WebGlRenderingContext;

mod common_funcs;
mod gl_setup;
mod programs;
mod shaders;
mod simulations;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub struct FolioClient {
    gl: WebGlRenderingContext,
    sim: GoL,
    canvas: HtmlCanvasElement,
}

#[wasm_bindgen]
impl FolioClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        let (gl, canvas) = gl_setup::init_webgl_ctx().unwrap();
        let gol = GoL::new(&gl, canvas.width() / 10, canvas.height() / 10);

        Self {
            gl,
            canvas,
            sim: gol,
        }
    }

    pub fn update(&mut self, _time: f32, _height: f32, _width: f32) -> Result<(), JsValue> {
        self.sim.update();
        Ok(())
    }

    pub fn render(&self) {
        self.gl.viewport(0, 0, self.gl.drawing_buffer_width(), self.gl.drawing_buffer_height());
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        let aspect: f32 = self.canvas.width() as f32 / self.canvas.height() as f32;
        self.sim.render(&self.gl, aspect);
    }
}
