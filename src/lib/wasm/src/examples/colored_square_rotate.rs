use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
// use web_sys::{WebGl2RenderingContext, WebGlBuffer};
// extern crate nalgebra_glm as glm;
// use crate::webgl::{compile_shader, draw, link_shader_program, ShaderInfo};
// use super::colored_square::{init_buffers, setup_shader_program, ShaderProgramInfo};

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

pub fn start(// context: &WebGl2RenderingContext,
    // canvas_height: f32,
    // canvas_width: f32,
) -> Result<(), JsValue> {
    // let buffers = init_buffers(&context);
    // let program = setup_shader_program(&context)?;

    // let info: ShaderInfo = ShaderInfo {
    //     program: &program.program,
    //     canvas_height,
    //     canvas_width,
    //     vertex_position: program.vertex_position,
    //     vertex_color: program.vertex_position,
    //     program_projection_matrix: &program.projection_matrix,
    //     program_model_view_matrix: &program.model_view_matrix,
    // };

    let st = get_current_time();

    {
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            let ct = get_current_time();
            // draw_scene(&context, &info, &buffers, st, ct);

            let delta = (ct - st) as f32;
            let text = format!("{:.4} sec passed.", delta);
            body().set_text_content(Some(&text));
            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));

        request_animation_frame(g.borrow().as_ref().unwrap());
    }

    Ok(())
}

// fn draw_scene(
//     context: &WebGl2RenderingContext,
//     info: &ShaderInfo,
//     (position_buffer, color_buffer): &(WebGlBuffer, WebGlBuffer),
//     initial_time: f64,
//     current_time: f64,
// ) {
//     let field_of_view = 45.0 * std::f32::consts::PI / 180.0;
//     let aspect = info.canvas_width / info.canvas_height;
//     let z_near = 0.1;
//     let z_far = 100.0;

//     let projection_matrix = glm::perspective(aspect, field_of_view, z_near, z_far);
//     let vec_projection_matrix = projection_matrix.iter().map(|v| *v).collect::<Vec<_>>();

//     let model_view_matrix =
//         glm::translate(&glm::Mat4::identity(), &glm::TVec3::new(-0.0, 0.0, -6.0));

//     let delta = (current_time - initial_time) as f32;

//     let model_view_matrix =
//         glm::rotate(&model_view_matrix, delta, &glm::TVec3::new(0.0, 0.0, 1.0));

//     let vec_model_view_matrix = model_view_matrix.iter().map(|v| *v).collect::<Vec<_>>();

//     draw(
//         context,
//         info,
//         position_buffer,
//         color_buffer,
//         &vec_projection_matrix[..],
//         &vec_model_view_matrix[..],
//     );
// }

fn get_current_time() -> f64 {
    js_sys::Date::now() / 1000.0 // sec
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn body() -> web_sys::HtmlElement {
    window()
        .document()
        .expect("should have a document on window")
        .body()
        .expect("document should have a body")
}
