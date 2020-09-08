use super::super::common_funcs as cf;
use js_sys::WebAssembly;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

pub struct Color2D {
    program: WebGlProgram,
    rect_vert_len: usize,
    rect_vertice_buffer: WebGlBuffer,
    u_color: WebGlUniformLocation,
    u_transform: WebGlUniformLocation,
}

impl Color2D {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let program = cf::link_program(
            &gl,
            super::super::shaders::vert::color_2d::SHADER,
            super::super::shaders::frag::color_2d::SHADER,
        )
        .unwrap();

        let vert_rect: [f32; 12] = [0., 1., 0., 0., 1., 1., 1., 1., 0., 0., 1., 0.];

        let mem_buff = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let vert_loc = vert_rect.as_ptr() as u32 / 4;
        let vert_array = js_sys::Float32Array::new(&mem_buff)
            .subarray(vert_loc, vert_loc + vert_rect.len() as u32);

        let buff_rect = gl.create_buffer().ok_or("failed to create buffer").unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buff_rect));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);

        Self {
            u_color: gl.get_uniform_location(&program, "uColor").unwrap(),
            u_transform: gl.get_uniform_location(&program, "uTransform").unwrap(),
            rect_vert_len: vert_rect.len(),
            rect_vertice_buffer: buff_rect,
            program: program,
        }
    }

    pub fn render(
        &self,
        gl: &WebGlRenderingContext,
        bottom: f32,
        top: f32,
        left: f32,
        right: f32,
        canvas_height: f32,
        canvas_width: f32,
    ) {
        gl.use_program(Some(&self.program));

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.rect_vertice_buffer));
        gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(0);

        gl.uniform4f(Some(&self.u_color), 0., 0.5, 1.0, 1.0);

        let translation_mat = cf::translation_matrix(
            2. * left / canvas_width - 1.,
            2. * bottom / canvas_height - 1.,
            0.,
        );

        let scale_mat = cf::scaling_matrix(
            2. * (right - left) / canvas_width,
            2. * (top - bottom) / canvas_height,
            0.,
        );
    }
}
