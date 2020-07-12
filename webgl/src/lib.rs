use wasm_bindgen::prelude::*;
use web_sys::*;

//type GL = web_sys::WebGlRenderingContext;

mod gl_setup;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s:&str);
}

#[wasm_bindgen]
pub struct FolioClient {
    gl: WebGlRenderingContext,
}

#[wasm_bindgen]
impl FolioClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        let gl = gl_setup::init_webgl_ctx().unwrap();

        Self {
            gl: gl,
        }
    }

    pub fn update(&mut self, _time: f32, _height: f32, _width: f32) -> Result<(), JsValue> {

        Ok(())
    }

    pub fn render(&self) {
       
    }
}
