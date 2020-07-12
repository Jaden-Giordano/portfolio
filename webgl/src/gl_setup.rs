use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::*;

type GL = web_sys::WebGlRenderingContext;

pub fn init_webgl_ctx() -> Result<GL, JsValue> {
    let win = window().unwrap();
    let doc = win.document().unwrap();
    let canvas = doc.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    let gl: GL = canvas.get_context("webgl")?.unwrap().dyn_into()?;

    gl.clear_color(0.0,0.0,0.0,1.0);

    Ok(gl)
}
