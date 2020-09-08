use web_sys::WebGlRenderingContext;

pub trait Simulation {
    fn start(&mut self);
    fn update(&mut self);
    fn render(&self, gl: &WebGlRenderingContext);
}
