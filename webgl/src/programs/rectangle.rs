use crate::common_funcs as cf;
use js_sys::{Float32Array, Uint16Array, WebAssembly};
use wasm_bindgen::JsCast;
use web_sys::{
    WebGlBuffer,
    WebGlProgram,
    WebGlRenderingContext as GL,
    WebGlUniformLocation,
};

const INDICES: [u16; 6] = [
    0, 1, 2,
    2, 3, 0,
];
const VERTICES: [f32; 12] = [
    0.0, 0.0, 0.0,
    1.0, 0.0, 0.0,
    1.0, 1.0, 0.0,
    0.0, 1.0, 0.0,
];

pub struct Rectangle {
    indices: WebGlBuffer,
    program: WebGlProgram,
    u_color: WebGlUniformLocation,
    u_scale: WebGlUniformLocation,
    u_translation: WebGlUniformLocation,
    vertices: WebGlBuffer,
}

impl Rectangle {
    pub fn new(gl: &GL) -> Self {
        let program = cf::link_program(
            &gl,
            crate::shaders::simple::VERT,
            crate::shaders::simple::FRAG,
        );
        let program = program.unwrap();

        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();

        let vertices_location = VERTICES.as_ptr() as u32 / 4;
        let vertex_array = Float32Array::new(&memory_buffer)
            .subarray(vertices_location, vertices_location + VERTICES.len() as u32);
        let vertices = gl.create_buffer().ok_or("failed to create vertex buffer").unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertices));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vertex_array, GL::STATIC_DRAW);

        let indices_location = INDICES.as_ptr() as u32;
        let index_array = Uint16Array::new(&memory_buffer)
            .subarray(indices_location, indices_location + INDICES.len() as u32);
        let indices = gl.create_buffer().ok_or("failed to create index buffer").unwrap();
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&indices));
        gl.buffer_data_with_array_buffer_view(GL::ELEMENT_ARRAY_BUFFER, &index_array, GL::STATIC_DRAW);

        let u_color = gl.get_uniform_location(&program, "u_Color").unwrap();
        let u_scale = gl.get_uniform_location(&program, "u_Scale").unwrap();
        let u_translation = gl.get_uniform_location(&program, "u_Translation").unwrap();

        Self {
            indices,
            program,
            u_color,
            u_scale,
            u_translation,
            vertices,
        }
    }

    pub fn render(
        &self,
        gl: &GL,
        aspect: f32
    ) {
        gl.use_program(Some(&self.program));

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.vertices));
        gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(0);

        gl.uniform4f(Some(&self.u_color), 1.0, 1.0, 1.0, 1.0);
        gl.uniform4f(Some(&self.u_translation), 0.0, 0.0, 0.0, 0.0);

        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.indices));

        gl.draw_elements_with_i32(
            GL::TRIANGLES,
            6,
            GL::UNSIGNED_SHORT,
            0,
        );
    }
}
