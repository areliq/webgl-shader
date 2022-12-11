extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
// use wasm_bindgen::JsValue;
// use wasm_bindgen::JsCast;

use web_sys::{HtmlCanvasElement, WebGl2RenderingContext, WebGlUniformLocation};
extern crate console_error_panic_hook;
extern crate nalgebra_glm as glm;

use crate::utils::log;
use crate::webgl::{compile_shader, get_context_with_canvas_by_id, link_shader_program};

#[wasm_bindgen]
pub struct MouseBox {
    context: WebGl2RenderingContext,
    canvas: HtmlCanvasElement,
    loc_position: u32,
    // loc_color: WebGlUniformLocation,
    loc_mouse_pos: Option<WebGlUniformLocation>,
    loc_time: Option<WebGlUniformLocation>,
}

#[wasm_bindgen]
impl MouseBox {
    pub fn new(
        id: &str,
        dynamic: bool,
        vertex_shader_source: &str,
        fragment_shader_source: &str,
    ) -> Self {
        console_error_panic_hook::set_once();

        let (context, canvas) = get_context_with_canvas_by_id(id).unwrap_or_else(|err| {
            log(&err);
            panic!("Failed to compile vertex shader");
        });

        log("MouseBox.new: context ok");

        resize_of(&context, &canvas);

        log("MouseBox.new: resize ok");

        let vertex_shader = compile_shader(
            &context,
            WebGl2RenderingContext::VERTEX_SHADER,
            vertex_shader_source,
        )
        .unwrap_or_else(|err| {
            log(&err);
            panic!("Failed to compile vertex shader");
        });

        log("MouseBox.new: vertex shader compiled");

        let fragment_shader = compile_shader(
            &context,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            fragment_shader_source,
        )
        .unwrap_or_else(|err| {
            log(&err);
            panic!("Failed to compile fragment shader");
        });

        log("MouseBox.new: fragment shader compiled");

        let program = link_shader_program(&context, &vertex_shader, &fragment_shader)
            .unwrap_or_else(|err| {
                log(&err);
                panic!("Failed to compile link shader");
            });

        log("MouseBox.new: shaders linked to program");

        context.use_program(Some(&program));

        log("MouseBox.new: use program ok");

        // set resolution (if not, it will become [0.0, 0.0])
        let loc_resolution = context.get_uniform_location(&program, "u_resolution");

        log("MouseBox.new: u_resolution location ok");

        match loc_resolution {
            None => {
                log("no location for u_resolution");
            }
            Some(loc) => {
                let viewport: [f32; 2] = [canvas.width() as f32, canvas.height() as f32];
                log("MouseBox.new: viewport ok");
                context.uniform2fv_with_f32_array(Some(&loc), &viewport);
                log("MouseBox.new: set viewport to u_resolution ok");
            }
        }

        // set mouse position
        let loc_mouse_pos = context.get_uniform_location(&program, "u_mouse_pos");

        log("MouseBox.new: u_mouse_pos location ok");

        match loc_mouse_pos {
            None => {
                log("no location for u_mouse_pos");
            }
            Some(ref loc_mouse) => {
                let initial_pos: [f32; 2] = [0.0, 0.0];
                context.uniform2fv_with_f32_array(Some(&loc_mouse), &initial_pos);
                log("MouseBox.new: set initial position to u_mouse_pos ok");
            }
        }

        let loc_time = if dynamic {
            // set time
            let loc_time = context
                .get_uniform_location(&program, "u_time")
                .unwrap_or_else(|| {
                    panic!("Failed to get uniform location: u_time");
                });

            log("MouseBox.new: u_time location ok");

            let current = get_current_sec() as f32;
            context.uniform1f(Some(&loc_time), current);

            log("MouseBox.new: set time to u_time ok");

            Some(loc_time)
        } else {
            None
        };

        // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.WebGlRenderingContext.html#method.get_attrib_location
        // https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getAttribLocation
        let loc_position: u32 = context
            .get_attrib_location(&program, "position")
            .try_into()
            .unwrap_or_else(|_| {
                // log(&err);
                panic!("Failed to get attribute location: position");
            });

        log("MouseBox.new: attribute location 'position' ok");

        // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.WebGlRenderingContext.html#method.get_uniform_location
        // https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getUniformLocation
        // let loc_color = context.get_uniform_location(&program, "color").unwrap();

        // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.WebGlRenderingContext.html#method.enable
        // https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/enable
        context.enable(WebGl2RenderingContext::DEPTH_TEST);

        log("MouseBox.new: DEPTH_TEST ok");

        MouseBox {
            context,
            canvas,
            loc_position,
            loc_mouse_pos,
            loc_time,
        }
    }

    fn bind_position_buffer(&self, positions: &[f32]) {
        let position_buffer = self
            .context
            .create_buffer()
            .ok_or("Failed to create buffer")
            .unwrap();

        self.context
            .bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&position_buffer));

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
            self.loc_position,
            num_components,
            data_type,
            normalize,
            stride,
            offset,
        );
    }

    // fn bind_color_buffer(&self, color: &[f32]) {
    //     // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.WebGlRenderingContext.html#method.uniform4fv_with_f32_array
    //     // https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform
    //     self.context.uniform4fv_with_f32_array(Some(&self.loc_color), &color);
    // }

    pub fn draw(&self) {
        resize_of(&self.context, &self.canvas);

        let positions = [
            // Triangle 1
            -1.0, -1.0, // left-bottom
            1.0, -1.0, // right-bottom
            -1.0, 1.0, // left-top
            // Triangle 2
            -1.0, 1.0, // left-top
            1.0, -1.0, // right-bottom
            1.0, 1.0, // right-top
        ];

        self.bind_position_buffer(&positions);

        // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.WebGlRenderingContext.html#method.draw_arrays
        // https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/drawArrays
        let offset = 0;
        let vertex_count = 6;
        self.context
            .draw_arrays(WebGl2RenderingContext::TRIANGLES, offset, vertex_count);
    }

    pub fn tick(&self, timestamp: f64, mouse_x: f64, mouse_y: f64) {
        match &self.loc_time {
            None => {}
            Some(loc) => {
                let current = timestamp as f32;
                self.context.uniform1f(Some(&loc), current);
            }
        }

        match &self.loc_mouse_pos {
            None => {}
            Some(loc) => {
                let next_pos: [f32; 2] = [mouse_x as f32, mouse_y as f32];
                self.context.uniform2fv_with_f32_array(Some(&loc), &next_pos);
            }
        }
    }
}

fn resize_of(context: &WebGl2RenderingContext, canvas: &HtmlCanvasElement) {
    let display_width: u32 = canvas.client_width().try_into().unwrap_or_else(|_| {
        panic!("Failed to get display width");
    });
    let display_height: u32 = canvas.client_height().try_into().unwrap();

    let canvas_width = canvas.width();
    let canvas_height = canvas.height();

    if canvas_width != display_width || canvas_height != display_height {
        canvas.set_width(display_width);
        canvas.set_height(display_height);
    }

    let new_width: i32 = canvas.width().try_into().unwrap();
    let new_height: i32 = canvas.height().try_into().unwrap();

    context.viewport(0, 0, new_width, new_height);
}

fn get_current_sec() -> f64 {
    js_sys::Date::now() / 1000.0 // sec
}

// fn get_current_msec() -> f64 {
//     js_sys::Date::now()  // msec
// }
