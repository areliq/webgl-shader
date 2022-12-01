extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
// use wasm_bindgen::JsValue;
// use wasm_bindgen::JsCast;

use web_sys::{WebGl2RenderingContext, HtmlCanvasElement};  // , WebGlUniformLocation
extern crate nalgebra_glm as glm;

use crate::webgl::{compile_shader, link_shader_program, get_context_with_canvas_by_id};
use crate::utils::{log};

static VERTEX_SHADER_SOURCE: &'static str = r#"#version 300 es
// The individual position vertex
in vec2 position;

void main() {
  // the gl_Position is the final position in clip space 
  // after the vertex shader modifies it
  gl_Position = vec4(position, 0.0, 1.0);
}
"#;

static FRAGMENT_SHADER_SOURCE: &'static str = r#"#version 300 es
precision mediump float;
out vec4 fragColor;
uniform vec2 u_resolution;

void main() {
  // vec2 res = vec2(1094, 929);
  // vec2 uv = (gl_FragCoord.xy * 2.0 - u_resolution) / min(u_resolution.x, u_resolution.y);
  vec2 uv = gl_FragCoord.xy / u_resolution.xy;
  // fragColor = vec4(uv.x, 0.0, uv.y, 1.0);
  fragColor = vec4(1.0, uv, 1.0);
}
"#;

#[wasm_bindgen]
pub struct HelloBoard {
    context: WebGl2RenderingContext,
    canvas: HtmlCanvasElement,
    loc_position: u32,
    // loc_color: WebGlUniformLocation,
}

#[wasm_bindgen]
impl HelloBoard {
    pub fn new(id: &str) -> Self {
        let (context, canvas) = get_context_with_canvas_by_id(id).unwrap();

        resize_of(&context, &canvas);

        let vertex_shader = compile_shader(
            &context,
            WebGl2RenderingContext::VERTEX_SHADER,
            VERTEX_SHADER_SOURCE,
        ).unwrap_or_else(|err| {
            log(&err);
            panic!("Failed to compile vertex shader");
        });
    
        let fragment_shader = compile_shader(
            &context,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            FRAGMENT_SHADER_SOURCE,
        ).unwrap_or_else(|err| {
            log(&err);
            panic!("Failed to compile fragment shader");
        });
    
        let program = link_shader_program(&context, &vertex_shader, &fragment_shader).unwrap();

        context.use_program(Some(&program));

        let loc_resolution = context.get_uniform_location(&program, "u_resolution").unwrap();
        // let resolution: js_sys::Float32Array = context
        //   .get_uniform(&program, &loc_resolution)
        //   .try_into()
        //   .unwrap();

        // let vec_resolution = resolution.to_vec();
        let viewport: [f32; 2] = [
             canvas.width() as f32, 
             canvas.height() as f32, 
        ];
        context.uniform2fv_with_f32_array(Some(&loc_resolution), &viewport);
        // log(&format!("resolution: {:?}", vec_resolution));

        // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.WebGlRenderingContext.html#method.get_attrib_location
        // https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getAttribLocation
        let loc_position: u32 = context.get_attrib_location(&program, "position").try_into().unwrap();

        // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.WebGlRenderingContext.html#method.get_uniform_location
        // https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getUniformLocation
        // let loc_color = context.get_uniform_location(&program, "color").unwrap();

        // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.WebGlRenderingContext.html#method.enable
        // https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/enable
        context.enable(WebGl2RenderingContext::DEPTH_TEST);

        HelloBoard {context, canvas, loc_position}  // , loc_color
    }

    fn bind_position_buffer(&self, positions: &[f32]) {
        let position_buffer = self.context
            .create_buffer()
            .ok_or("Failed to create buffer")
            .unwrap();

        self.context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&position_buffer));

        unsafe {
            let positions_array_buffer_view = js_sys::Float32Array::view(positions);

            self.context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &positions_array_buffer_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        self.context.enable_vertex_attrib_array(self.loc_position);

        let num_components = 2;
        let data_type: u32 = WebGl2RenderingContext::FLOAT;
        let normalize = false;
        let stride = 0;
        let offset = 0;

        self.context.vertex_attrib_pointer_with_i32(
            self.loc_position, num_components, data_type, normalize, stride, offset
        );
    }

    // fn bind_color_buffer(&self, color: &[f32]) {
    //     // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.WebGlRenderingContext.html#method.uniform4fv_with_f32_array
    //     // https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform
    //     self.context.uniform4fv_with_f32_array(Some(&self.loc_color), &color);
    // }
    
    pub fn draw(&self) {
        // self.resize();

        log(&format!("viewport: {}", WebGl2RenderingContext::VIEWPORT));

        let positions = [
            // Triangle 1
            -1.0, -1.0,  // left-bottom
             1.0, -1.0,  // right-bottom
            -1.0,  1.0,  // left-top
            
            // Triangle 2
            -1.0,  1.0,  // left-top
             1.0, -1.0,  // right-bottom
             1.0,  1.0,  // right-top
        ];

        // let color = [1.0, 0.0, 0.5, 1.0] as [f32; 4];

        self.bind_position_buffer(&positions);
        // self.bind_color_buffer(&color);
        
        // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.WebGlRenderingContext.html#method.draw_arrays
        // https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/drawArrays
        let offset = 0;
        let vertex_count = 6;
        self.context.draw_arrays(WebGl2RenderingContext::TRIANGLES, offset, vertex_count);
    }

    // TODO: refactor
    pub fn resize(&self) {

        let display_width: u32 = self.canvas.client_width().try_into().unwrap();
        let display_height: u32 = self.canvas.client_height().try_into().unwrap();
    
        let canvas_width = self.canvas.width();
        let canvas_height = self.canvas.height();

        log(&format!("display w: {} / canvas w: {}", display_width, canvas_width));
        log(&format!("display h: {} / canvas h: {}", display_height, canvas_height));
    
        if canvas_width != display_width || canvas_height != display_height {
            self.canvas.set_width(display_width);
            self.canvas.set_height(display_height);
        }

        let new_width: i32 = self.canvas.width().try_into().unwrap();
        let new_height: i32 = self.canvas.height().try_into().unwrap();

        log(&format!("new w: {} / h: {}", new_width, new_height));
    
        self.context.viewport(0, 0, new_width, new_height);
    }
}

fn resize_of(context: &WebGl2RenderingContext, canvas: &HtmlCanvasElement) {

    let display_width: u32 = canvas.client_width().try_into().unwrap();
    let display_height: u32 = canvas.client_height().try_into().unwrap();

    let canvas_width = canvas.width();
    let canvas_height = canvas.height();

    log(&format!("display w: {} / canvas w: {}", display_width, canvas_width));
    log(&format!("display h: {} / canvas h: {}", display_height, canvas_height));

    if canvas_width != display_width || canvas_height != display_height {
        canvas.set_width(display_width);
        canvas.set_height(display_height);
    }

    let new_width: i32 = canvas.width().try_into().unwrap();
    let new_height: i32 = canvas.height().try_into().unwrap();

    log(&format!("new w: {} / h: {}", new_width, new_height));

    context.viewport(0, 0, new_width, new_height);
}

// todo: check len(positions) / num_components == vertex_count 