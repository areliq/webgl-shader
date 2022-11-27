extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
extern crate nalgebra_glm as glm;

use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlUniformLocation};

use super::colored_square::{init_buffers, setup_shader_program};
use crate::webgl::get_context_by_id;

#[wasm_bindgen]
pub struct RotatingSquare {
    context: WebGl2RenderingContext,
    canvas_width: f32,
    canvas_height: f32,
    position_buffer: WebGlBuffer,
    color_buffer: WebGlBuffer,
    shader_program: WebGlProgram,
    shader_vertex_position: u32,
    shader_vertex_color: u32,
    shader_projection_matrix: WebGlUniformLocation,
    shader_model_view_matrix: WebGlUniformLocation,
    delta: f32,
    start_at: f64,
}

#[wasm_bindgen]
impl RotatingSquare {
    pub fn new(id: &str) -> RotatingSquare {
        let (context, canvas) = get_context_by_id(id).unwrap();
        let (position_buffer, color_buffer) = init_buffers(&context);
        let program = setup_shader_program(&context).unwrap();

        RotatingSquare {
            context,
            canvas_width: canvas.width,
            canvas_height: canvas.height,
            position_buffer,
            color_buffer,
            shader_program: program.program,
            shader_vertex_position: program.vertex_position,
            shader_vertex_color: program.vertex_color,
            shader_projection_matrix: program.projection_matrix,
            shader_model_view_matrix: program.model_view_matrix,

            delta: 0.0,
            start_at: get_current_time(),
        }
    }

    pub fn render(&self) {
        let field_of_view = 45.0 * std::f32::consts::PI / 180.0;
        let aspect = self.canvas_width / self.canvas_height;
        let z_near = 0.1;
        let z_far = 100.0;

        let projection_matrix = glm::perspective(aspect, field_of_view, z_near, z_far);

        let model_view_matrix =
            glm::translate(&glm::Mat4::identity(), &glm::TVec3::new(-0.0, 0.0, -6.0));

        let model_view_matrix = glm::rotate(
            &model_view_matrix,
            self.delta,
            &glm::TVec3::new(0.0, 0.0, 1.0),
        );

        let vec_projection_matrix = projection_matrix.iter().map(|v| *v).collect::<Vec<_>>();
        let vec_model_view_matrix = model_view_matrix.iter().map(|v| *v).collect::<Vec<_>>();

        self.draw(&vec_projection_matrix[..], &vec_model_view_matrix[..])
    }

    pub fn tick(&mut self) {
        self.delta = (get_current_time() - self.start_at) as f32;
    }

    fn draw(&self, vec_projection_matrix: &[f32], vec_model_view_matrix: &[f32]) {
        self.context.clear_color(0.0, 0.0, 0.0, 1.0); // black, fully-opaque
        self.context.clear_depth(1.0);
        self.context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        self.context.enable(WebGl2RenderingContext::DEPTH_TEST);
        self.context.depth_func(WebGl2RenderingContext::LEQUAL);

        self.context.clear(
            WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
        );

        // Tell WebGL how to pull out the positions from the position
        // buffer into the VertexPosition attribute
        {
            let num_components = 2;
            let data_type: u32 = WebGl2RenderingContext::FLOAT;
            let normalize = false;
            let stride = 0;
            let offset = 0;

            self.context.bind_buffer(
                WebGl2RenderingContext::ARRAY_BUFFER,
                Some(&self.position_buffer),
            );
            self.context.vertex_attrib_pointer_with_i32(
                self.shader_vertex_position,
                num_components,
                data_type,
                normalize,
                stride,
                offset,
            );

            self.context
                .enable_vertex_attrib_array(self.shader_vertex_position);
        }

        // Tell WebGL how to pull out the colors from the color buffer
        // into the VertexColor attribute.
        {
            let num_components = 4;
            let data_type: u32 = WebGl2RenderingContext::FLOAT;
            let normalize = false;
            let stride = 0;
            let offset = 0;

            self.context.bind_buffer(
                WebGl2RenderingContext::ARRAY_BUFFER,
                Some(&self.color_buffer),
            );
            self.context.vertex_attrib_pointer_with_i32(
                self.shader_vertex_color,
                num_components,
                data_type,
                normalize,
                stride,
                offset,
            );
            self.context
                .enable_vertex_attrib_array(self.shader_vertex_color);
        }

        // Tell WebGL to use our program when drawing
        self.context.use_program(Some(&self.shader_program));

        // Set the shader uniforms
        self.context.uniform_matrix4fv_with_f32_array(
            Some(&self.shader_projection_matrix),
            false,
            &vec_projection_matrix,
        );

        self.context.uniform_matrix4fv_with_f32_array(
            Some(&self.shader_model_view_matrix),
            false,
            &vec_model_view_matrix,
        );

        let offset = 0;
        let vertex_count = 4;
        // let data_type = WebGl2RenderingContext::UNSIGNED_SHORT;
        self.context
            .draw_arrays(WebGl2RenderingContext::TRIANGLE_STRIP, offset, vertex_count);
    }
}

fn get_current_time() -> f64 {
    js_sys::Date::now() / 1000.0 // sec
}
