use wasm_bindgen::prelude::*;
use web_sys::*;

use crate::programs::Rectangle;

type GL = web_sys::WebGlRenderingContext;

mod common_funcs;
mod gl_setup;
mod programs;
mod shaders;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub struct FolioClient {
    gl: WebGlRenderingContext,
    program: Rectangle,
    canvas: HtmlCanvasElement,
}

#[wasm_bindgen]
impl FolioClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        let (gl, canvas) = gl_setup::init_webgl_ctx().unwrap();
        let program = Rectangle::new(&gl);

        Self {
            gl,
            program,
            canvas,
        }
    }

    pub fn update(&mut self, _time: f32, _height: f32, _width: f32) -> Result<(), JsValue> {
        Ok(())
    }

    pub fn render(&self) {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        let aspect: f32 = self.canvas.width() as f32 / self.canvas.height() as f32;
        self.program.render(&self.gl, aspect);
    }
}
