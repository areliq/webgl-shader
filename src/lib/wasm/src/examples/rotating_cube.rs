extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
// use wasm_bindgen::JsValue;
use wasm_bindgen::JsCast;

use web_sys::{HtmlCanvasElement, WebGlBuffer, WebGl2RenderingContext, WebGlUniformLocation, WebGlProgram, WebGlShader};
extern crate console_error_panic_hook;
extern crate nalgebra_glm as glm;

// use crate::utils::log;


static VS_SRC: &'static str = r#"#version 300 es
in vec4 aVertexPosition;
in vec4 aVertexColor;

uniform mat4 uModelViewMatrix;
uniform mat4 uProjectionMatrix;

out vec4 vColor;

void main() {
  gl_Position = uProjectionMatrix * uModelViewMatrix * aVertexPosition;
  // gl_PointSize = 4.0;
  vColor = aVertexColor;
}
"#;

static FS_SRC: &'static str = r#"#version 300 es
precision mediump float;
in vec4 vColor;
out vec4 fragColor;

void main() {
  fragColor = vColor;
}
"#;

// static VERTEX_SHADER_SOURCE: &'static str = r#"
//   attribute vec4 aVertexPosition;
//   attribute vec4 aVertexColor;
//   uniform mat4 uModelViewMatrix;
//   uniform mat4 uProjectionMatrix;
//   varying lowp vec4 vColor;
//   void main() {
//     gl_Position = uProjectionMatrix * uModelViewMatrix * aVertexPosition;
//     vColor = aVertexColor;
//   }
// "#;

// static FRAGMENT_SHADER_SOURCE: &'static str = r#"
//   varying lowp vec4 vColor;
//   void main() {
//     gl_FragColor = vColor;
//   }
// "#;

// static SQUARE_POSITIONS: [f32; 12] = [
//     -1.0,  1.0, 1.0,
//      1.0,  1.0, 1.0,
//     -1.0, -1.0, 1.0,
//      1.0, -1.0, 1.0,
// ];

static CUBE_POSITIONS: [f32; 72] = [
    // Front face: White
    -1.0, -1.0,  1.0,  // debug: skyblue
     1.0, -1.0,  1.0,  // debug: red
     1.0,  1.0,  1.0,  // debug: green
    -1.0,  1.0,  1.0,  // debug: blue
    
    // Back face: Red
    -1.0, -1.0, -1.0, 
    -1.0,  1.0, -1.0, 
     1.0,  1.0, -1.0, 
     1.0, -1.0, -1.0,

    // Top face
    -1.0,  1.0, -1.0, 
    -1.0,  1.0,  1.0, 
     1.0,  1.0,  1.0, 
     1.0,  1.0, -1.0,  // 11

    // Bottom face
    -1.0, -1.0, -1.0, 
     1.0, -1.0, -1.0, 
     1.0, -1.0,  1.0, 
    -1.0, -1.0,  1.0,

    // Right face
     1.0, -1.0, -1.0, 
     1.0,  1.0, -1.0, 
     1.0,  1.0,  1.0, 
     1.0, -1.0,  1.0,

    // Left face: Purple
    -1.0, -1.0, -1.0, 
    -1.0, -1.0,  1.0, 
    -1.0,  1.0,  1.0, 
    -1.0,  1.0, -1.0,
];

// static SQUARE_INDICES: [u16; 6] = [
//     0, 1, 2, 1, 2, 3
// ];


// This array defines each face as two triangles, using the
// indices into the vertex array to specify each triangle's
// position.
static CUBE_INDICES: [u16; 36] = [
     0,  1,  2,  0,  2,  3, // front
     4,  5,  6,  4,  6,  7, // back
     8,  9, 10,  8, 10, 11, // top: green
    12, 13, 14, 12, 14, 15, // bottom
    16, 17, 18, 16, 18, 19, // right
    20, 21, 22, 20, 22, 23, // left: purple
    // 20, 21, 22, 23, 20, 21, // left: purple
];


fn cube_colors() -> Vec<f32> {
    let face_colors: [[f32; 4]; 6] = [
        [0.0, 1.0, 1.0, 1.0], // Front face: white
        [1.0, 0.0, 0.0, 1.0], // Back face: red
        [0.0, 1.0, 0.0, 1.0], // Top face: green
        [0.0, 0.0, 1.0, 1.0], // Bottom face: blue
        [1.0, 1.0, 0.0, 1.0], // Right face: yellow
        [1.0, 0.0, 1.0, 1.0], // Left face: purple
    ];

    face_colors.iter()
        .map(|color| color.repeat(4))
        .collect::<Vec<_>>()
        .concat()

    // let face_colors: [f32; 16] = [
    //     0.0, 1.0, 1.0, 1.0, // Front face: white
    //     1.0, 0.0, 0.0, 1.0, // Back face: red
    //     0.0, 1.0, 0.0, 1.0, // Top face: green
    //     0.0, 0.0, 1.0, 1.0, // Bottom face: blue
    //         // [1.0, 1.0, 0.0, 1.0], // Right face: yellow
    //         // [1.0, 0.0, 1.0, 1.0], // Left face: purple
    // ];
    
    // face_colors.to_vec() // iter()
        // .map(|color| color.repeat(4))
        // .collect::<Vec<_>>()
        // .concat()
}


#[wasm_bindgen]
pub struct RotatingCube {
    context: WebGl2RenderingContext,
    canvas: HtmlCanvasElement,
    // loc_vertex_position: u32,
    // loc_vertex_color: u32,
    // loc_model_view_matrix: WebGlUniformLocation,
    // loc_projection_matrix: WebGlUniformLocation,
    location: Location,
    delta: f32,
}

#[wasm_bindgen]
impl RotatingCube {
    pub fn new(
        id: &str,
    ) -> Self {
        console_error_panic_hook::set_once();

        let (ctx, canvas) = get_context_by_id(id).unwrap();
        let vs = compile_shader(&ctx, WebGl2RenderingContext::VERTEX_SHADER, VS_SRC).unwrap();
        let fs = compile_shader(&ctx, WebGl2RenderingContext::FRAGMENT_SHADER, FS_SRC).unwrap();
        let program = link_shader_program(&ctx, &vs, &fs).unwrap();
        ctx.use_program(Some(&program));

        let location = get_locations(&ctx, &program).unwrap();

        // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.WebGlRenderingContext.html#method.enable
        // https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/enable
        ctx.enable(WebGl2RenderingContext::DEPTH_TEST);

        RotatingCube { context: ctx, canvas, location, delta: 0.0, }
    }

    

    // fn bind_color_buffer(&self, color: &[f32]) {
    //     // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.WebGlRenderingContext.html#method.uniform4fv_with_f32_array
    //     // https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform
    //     self.context.uniform4fv_with_f32_array(Some(&self.loc_color), &color);
    // }

    pub fn tick(&mut self, delta: f64) {
        self.delta = delta as f32;
    }

    pub fn draw(&self) {
        resize_of(&self.context, &self.canvas);

        // clear canvas
        self.context.clear_color(0.0, 0.0, 0.0, 1.0); // black, fully-opaque
        self.context.clear_depth(1.0);
        self.context.enable(WebGl2RenderingContext::DEPTH_TEST);
        self.context.depth_func(WebGl2RenderingContext::LEQUAL);

        self.context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);

        // let vao = self.context.create_vertex_array().unwrap();
        // self.context.bind_vertex_array(Some(&vao));

        let buf_position = self.bind_array_buffer(&CUBE_POSITIONS);
        let buf_colors = self.bind_array_buffer(&cube_colors()[..]);
        let buf_indices = self.bind_index_buffer(&CUBE_INDICES);

        self.set_vertex_attribute(self.location.vertex_position, 3, &buf_position);
        self.set_vertex_attribute(self.location.vertex_color, 4, &buf_colors);

        // why need to bind again?
        self.context
            .bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&buf_indices));

       
        // setup matrix
        let field_of_view = 45.0 * std::f32::consts::PI / 180.0;
        let aspect = self.canvas.client_width() as f32 / self.canvas.client_height() as f32;
        let z_near = 0.1;
        let z_far = 100.0;

        // https://docs.rs/nalgebra-glm/latest/nalgebra_glm/fn.perspective.html
        let projection_matrix = glm::perspective(aspect, field_of_view, z_near, z_far);
        // let projection_matrix = glm::Mat4::identity();

        // // https://docs.rs/nalgebra-glm/latest/nalgebra_glm/fn.translate.html
        let model_view_matrix =
            glm::translate(&glm::Mat4::identity(), &glm::TVec3::new(-0.0, 0.0, -6.0));
        // let model_view_matrix = glm::Mat4::identity();

        // https://docs.rs/nalgebra-glm/latest/nalgebra_glm/fn.rotate.html
        let model_view_matrix = glm::rotate(
            &model_view_matrix,
            self.delta,
            &glm::TVec3::new(0.0, 0.0, 1.0),  // rotate around axis Z
        );

        let model_view_matrix = glm::rotate(
            &model_view_matrix,
            self.delta * 0.7,
            &glm::TVec3::new(0.0, 1.0, 0.0),  // rotate around axis Y
        );

        let model_view_matrix = glm::rotate(
            &model_view_matrix,
            self.delta * 0.3,
            &glm::TVec3::new(1.0, 0.0, 0.0),  // rotate around axis X
        );

        let vec_projection_matrix = projection_matrix.iter().map(|v| *v).collect::<Vec<_>>();
        let vec_model_view_matrix = model_view_matrix.iter().map(|v| *v).collect::<Vec<_>>();

        self.context.uniform_matrix4fv_with_f32_array(
            Some(&self.location.projection_matrix),
            false,
            &vec_projection_matrix,
        );
    
        self.context.uniform_matrix4fv_with_f32_array(
            Some(&self.location.model_view_matrix),
            false,
            &vec_model_view_matrix,
        );
    
        // draw
        let offset = 0;
        let vertex_count: i32 = CUBE_INDICES.len().try_into().unwrap();
        let data_type = WebGl2RenderingContext::UNSIGNED_SHORT;
        let instance_count = 1;

        // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.WebGl2RenderingContext.html#method.draw_elements_instanced_with_f64
        self.context.draw_elements_instanced_with_i32(
            WebGl2RenderingContext::TRIANGLES, 
            // WebGl2RenderingContext::POINTS,
            // WebGl2RenderingContext::LINES,
            vertex_count, 
            data_type, 
            offset,
            instance_count,
        );
    }

    fn bind_array_buffer(&self, array: &[f32]) -> WebGlBuffer {
        let buffer = self.context.create_buffer().unwrap();

        self.context
            .bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

        unsafe {
            let array_buffer_view = js_sys::Float32Array::view(array);

            self.context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &array_buffer_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        buffer
    }

    fn set_vertex_attribute(&self, location: u32, num_components: i32, buffer: &WebGlBuffer) {
        self.context.enable_vertex_attrib_array(location);

        let data_type: u32 = WebGl2RenderingContext::FLOAT;
        let normalize = false;
        let stride = 0;
        let offset = 0;


        // why need to bind again?
        self.context
            .bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(buffer));


        // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.WebGl2RenderingContext.html#method.vertex_attrib_pointer_with_i32
        // https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/vertexAttribIPointer
        self.context.vertex_attrib_pointer_with_i32(
            location,
            num_components,
            data_type,
            normalize,
            stride,
            offset,
        );        
    }

    fn bind_index_buffer(&self, indices: &[u16]) -> WebGlBuffer {
        let buffer = self.context.create_buffer().unwrap();

        self.context
            .bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&buffer));

        unsafe {
            let index_buffer_view = js_sys::Uint16Array::view(indices);
    
            self.context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                &index_buffer_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        buffer
    }
}

fn resize_of(context: &WebGl2RenderingContext, canvas: &HtmlCanvasElement) {
    let display_width: u32 = canvas.client_width().try_into().unwrap_or_else(|_| {
        panic!("Failed to get display width");
    });
    let display_height: u32 = canvas.client_height().try_into().unwrap_or_else(|_| {
        panic!("Failed to get display height");
    });

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

// fn get_current_sec() -> f64 {
//     js_sys::Date::now() / 1000.0 // sec
// }

fn get_context_by_id(
    id: &str,
) -> Result<(WebGl2RenderingContext, web_sys::HtmlCanvasElement), String> {
    let document = web_sys::window().unwrap().document().unwrap();

    let canvas: web_sys::HtmlCanvasElement = document
        .get_element_by_id(id)
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    let context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .unwrap();

    Ok((context, canvas))
}

fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    let compile_success = context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false);

    if compile_success {
        Ok(shader)
    } else {
        let msg = context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader"));
        Err(msg)
    }
}

fn link_shader_program(
    context: &WebGl2RenderingContext,
    vertex_shader: &WebGlShader,
    fragment_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vertex_shader);
    context.attach_shader(&program, fragment_shader);
    context.link_program(&program);

    let link_success = context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false);

    if link_success {
        Ok(program)
    } else {
        let msg = context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object"));
        Err(msg)
    }
}


struct Location {
    vertex_position: u32,
    vertex_color: u32,
    model_view_matrix: WebGlUniformLocation,
    projection_matrix: WebGlUniformLocation,
}

fn get_locations(context: &WebGl2RenderingContext, program: &WebGlProgram) -> Result<Location, String> {
        // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.WebGlRenderingContext.html#method.get_attrib_location
        // https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getAttribLocation
        let loc_vertex_position: u32 = context
            .get_attrib_location(&program, "aVertexPosition")
            .try_into()
            .unwrap(); //_or_else(|| String::from("Failed to get attribute location: aVertexPosition"));

        let loc_vertex_color: u32 = context
            .get_attrib_location(&program, "aVertexColor")
            .try_into()
            .unwrap(); // _or_else(|| String::from("Failed to get attribute location: aVertexColor"));

        // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.WebGlRenderingContext.html#method.get_uniform_location
        // https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getUniformLocation
        let loc_model_view_matrix = context
            .get_uniform_location(&program, "uModelViewMatrix")
            .unwrap(); // _or_else(|| String::from("Failed to get uniform location: uModelViewMatrix"));

        let loc_projection_matrix = context
            .get_uniform_location(&program, "uProjectionMatrix")
            .unwrap(); // _or_else(|| String::from("Failed to get uniform location: uProjectionMatrix"));

        Ok(Location {
            vertex_position: loc_vertex_position, 
            vertex_color: loc_vertex_color, 
            model_view_matrix: loc_model_view_matrix, 
            projection_matrix: loc_projection_matrix,
        })
}
