use wasm_bindgen::JsCast;
use web_sys::{
    WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlShader, WebGlUniformLocation,
};

pub struct CanvasProperties {
    pub height: f32,
    pub width: f32,
}

pub fn get_context_by_id(id: &str) -> Result<(WebGl2RenderingContext, CanvasProperties), String> {
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

    let properties = CanvasProperties {
        height: canvas.client_height() as f32,
        width: canvas.client_width() as f32,
    };

    Ok((context, properties))
}

pub fn get_context_with_canvas_by_id(id: &str) -> Result<(WebGl2RenderingContext, web_sys::HtmlCanvasElement), String> {
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

pub fn compile_shader(
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

pub fn link_shader_program(
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

pub struct ShaderInfo<'a> {
    pub program: &'a WebGlProgram,
    pub canvas_width: f32,
    pub canvas_height: f32,
    pub vertex_position: u32,
    pub vertex_color: u32,
    pub program_projection_matrix: &'a WebGlUniformLocation,
    pub program_model_view_matrix: &'a WebGlUniformLocation,
}

pub fn draw(
    context: &WebGl2RenderingContext,
    info: &ShaderInfo,
    position_buffer: &WebGlBuffer,
    color_buffer: &WebGlBuffer,
    vec_projection_matrix: &[f32],
    vec_model_view_matrix: &[f32],
) {
    context.clear_color(0.0, 0.0, 0.0, 1.0); // black, fully-opaque
    context.clear_depth(1.0);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    context.enable(WebGl2RenderingContext::DEPTH_TEST);
    context.depth_func(WebGl2RenderingContext::LEQUAL);

    context
        .clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);

    // Tell WebGL how to pull out the positions from the position
    // buffer into the VertexPosition attribute
    {
        let num_components = 2;
        let data_type: u32 = WebGl2RenderingContext::FLOAT;
        let normalize = false;
        let stride = 0;
        let offset = 0;

        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&position_buffer));
        context.vertex_attrib_pointer_with_i32(
            info.vertex_position,
            num_components,
            data_type,
            normalize,
            stride,
            offset,
        );

        context.enable_vertex_attrib_array(info.vertex_position);
    }

    // Tell WebGL how to pull out the colors from the color buffer
    // into the VertexColor attribute.
    {
        let num_components = 4;
        let data_type: u32 = WebGl2RenderingContext::FLOAT;
        let normalize = false;
        let stride = 0;
        let offset = 0;
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&color_buffer));
        context.vertex_attrib_pointer_with_i32(
            info.vertex_color,
            num_components,
            data_type,
            normalize,
            stride,
            offset,
        );
        context.enable_vertex_attrib_array(info.vertex_color);
    }

    // Tell WebGL to use our program when drawing
    context.use_program(Some(info.program));

    // Set the shader uniforms
    context.uniform_matrix4fv_with_f32_array(
        Some(info.program_projection_matrix),
        false,
        &vec_projection_matrix,
    );

    context.uniform_matrix4fv_with_f32_array(
        Some(info.program_model_view_matrix),
        false,
        &vec_model_view_matrix,
    );

    let offset = 0;
    let vertex_count = 4;
    // let data_type = WebGl2RenderingContext::UNSIGNED_SHORT;
    context.draw_arrays(WebGl2RenderingContext::TRIANGLE_STRIP, offset, vertex_count);
}
