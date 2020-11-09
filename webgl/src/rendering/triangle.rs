use crate::common_funcs as cf;
use js_sys::{Float32Array, Uint16Array, WebAssembly};
use wasm_bindgen::JsCast;
use web_sys::{WebGlBuffer, WebGlProgram, WebGlRenderingContext as GL, WebGlUniformLocation};

const INDICES: [u16; 3] = [0, 1, 2];
//const VERTICES: [f32; 9] = [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0];

pub struct Triangle {
    indices: WebGlBuffer,
    program: WebGlProgram,
    u_color: WebGlUniformLocation,
    u_scale: WebGlUniformLocation,
    u_translation: WebGlUniformLocation,
    u_rotation: WebGlUniformLocation,
    vertex_buffer: WebGlBuffer,
}

impl Triangle {
    pub fn new(gl: &GL, vertices: [f32; 6]) -> Self {
        let program = cf::link_program(
            &gl,
            crate::shaders::simple::VERT,
            crate::shaders::simple::FRAG,
        )
        .unwrap();

        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();

        let vertices_location = vertices.as_ptr() as u32 / 4;
        let vertex_array = Float32Array::new(&memory_buffer)
            .subarray(vertices_location, vertices_location + vertices.len() as u32);
        let vertex_buffer = gl
            .create_buffer()
            .ok_or("failed to create vertex buffer")
            .unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vertex_array, GL::STATIC_DRAW);

        let indices_location = INDICES.as_ptr() as u32 / 2;
        let index_array = Uint16Array::new(&memory_buffer)
            .subarray(indices_location, indices_location + INDICES.len() as u32);
        let indices = gl
            .create_buffer()
            .ok_or("failed to create index buffer")
            .unwrap();
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&indices));
        gl.buffer_data_with_array_buffer_view(
            GL::ELEMENT_ARRAY_BUFFER,
            &index_array,
            GL::STATIC_DRAW,
        );

        let u_color = gl.get_uniform_location(&program, "u_Color").unwrap();
        let u_scale = gl.get_uniform_location(&program, "u_Scale").unwrap();
        let u_translation = gl.get_uniform_location(&program, "u_Translation").unwrap();
        let u_rotation = gl.get_uniform_location(&program, "u_Rotation").unwrap(); //were going to do rotation in the shader, is faster.

        Self {
            indices,
            program,
            u_color,
            u_scale,
            u_translation,
            u_rotation,
            vertex_buffer,
        }
    }

    pub fn render(&self, gl: &GL, x: f32, y: f32, width: f32, height: f32, color: [f32; 4]) {
        gl.use_program(Some(&self.program));

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.vertex_buffer));
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.indices));

        gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(0);

        gl.uniform4f(Some(&self.u_color), color[0], color[1], color[2], color[3]);
        gl.uniform4f(Some(&self.u_scale), width, height, 1.0, 1.0);
        gl.uniform4f(Some(&self.u_translation), x, y, 0.0, 0.0);
        gl.uniform1f(Some(&self.u_rotation), 3.14 / 2.0);

        gl.draw_elements_with_i32(GL::TRIANGLES, INDICES.len() as i32, GL::UNSIGNED_SHORT, 0);
    }
}
