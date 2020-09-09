use web_sys::WebGlRenderingContext;

pub trait Simulation {
    fn update(&mut self);
    fn render(&self, gl: &WebGlRenderingContext);
}
