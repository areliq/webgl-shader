extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
// use wasm_bindgen::JsValue;

use web_sys::{WebGl2RenderingContext, WebGlUniformLocation};
extern crate nalgebra_glm as glm;

use crate::webgl::{compile_shader, link_shader_program, get_context_by_id};

#[wasm_bindgen]
pub fn render_boxes_to_clipspace_directly(id: &str) {
    let csbox = ClipSpaceBox::new(id);

    // Draw a red box in the middle
    csbox.draw(&Square {
        t: 0.5,
        b: -0.5,
        l: -0.5,
        r: 0.5,

        depth: 0.0,
        color: [1.0, 0.4, 0.4, 1.0],  // red
    });

    // Draw a green box up top
    csbox.draw(&Square {
        t: 0.9,
        b: 0.0,
        l: -0.9,
        r: 0.9,

        depth: 0.5,
        color: [0.4, 1.0, 0.4, 1.0],  // green
    });

    // This box doesn't get drawn because it's outside of clip space. 
    // The depth is outside of the -1.0 to 1.0 range.
    csbox.draw(&Square {
        t: 1.0,
        b: -1.0,
        l: -1.0,
        r: 1.0,

        depth: -1.5,
        color: [0.4, 0.4, 1.0, 1.0],  // blue
    });
}


struct ClipSpaceBox {
    context: WebGl2RenderingContext,
    loc_position: u32,
    loc_color: WebGlUniformLocation,
}

static VERTEX_SHADER_SOURCE: &'static str = r#"
// The individual position vertex
attribute vec3 position;

void main() {  
  // the gl_Position is the final position in clip space 
  // after the vertex shader modifies it
  gl_Position = vec4(position, 1.0);
}
"#;

static FRAGMENT_SHADER_SOURCE: &'static str = r#"
precision mediump float;
uniform vec4 color;

void main() {
  gl_FragColor = color;
}
"#;


struct Square {
    t: f32,  // top
    b: f32,  // bottom
    l: f32,  // left
    r: f32,  //right
    depth: f32,
    color: [f32; 4],
}

impl ClipSpaceBox {
    pub fn new(id: &str) -> Self {
        let (context, _canvas) = get_context_by_id(id).unwrap();
        
        let vertex_shader = compile_shader(
            &context,
            WebGl2RenderingContext::VERTEX_SHADER,
            VERTEX_SHADER_SOURCE,
        ).unwrap();
    
        let fragment_shader = compile_shader(
            &context,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            FRAGMENT_SHADER_SOURCE,
        ).unwrap();
    
        let program = link_shader_program(&context, &vertex_shader, &fragment_shader).unwrap();

        context.use_program(Some(&program));

        // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.WebGlRenderingContext.html#method.get_attrib_location
        // https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getAttribLocation
        let loc_position: u32 = context.get_attrib_location(&program, "position").try_into().unwrap();

        // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.WebGlRenderingContext.html#method.get_uniform_location
        // https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getUniformLocation
        let loc_color = context.get_uniform_location(&program, "color").unwrap();

        // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.WebGlRenderingContext.html#method.enable
        // https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/enable
        context.enable(WebGl2RenderingContext::DEPTH_TEST);

        ClipSpaceBox {context, loc_position, loc_color}
    }
    
    pub fn draw(&self, sq: &Square) {
        let positions = [
            // Triangle 1
            sq.l, sq.b, sq.depth,
            sq.r, sq.b, sq.depth,
            sq.l, sq.t, sq.depth,
            
            // Triangle 2
            sq.l, sq.t, sq.depth,
            sq.r, sq.b, sq.depth,
            sq.r, sq.t, sq.depth,
        ];

        let position_buffer = self.context
            .create_buffer()
            .ok_or("Failed to create buffer")
            .unwrap();

        self.context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&position_buffer));

        unsafe {
            let positions_array_buffer_view = js_sys::Float32Array::view(&positions);

            self.context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &positions_array_buffer_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        self.context.enable_vertex_attrib_array(self.loc_position);
        self.context.vertex_attrib_pointer_with_i32(self.loc_position, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);

        // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.WebGlRenderingContext.html#method.uniform4fv_with_f32_array
        // https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform
        self.context.uniform4fv_with_f32_array(Some(&self.loc_color), &sq.color);
    
        
        // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.WebGlRenderingContext.html#method.draw_arrays
        // https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/drawArrays
        let offset = 0;
        let vertex_count = 6;
        self.context.draw_arrays(WebGl2RenderingContext::TRIANGLES, offset, vertex_count);
    }
}
