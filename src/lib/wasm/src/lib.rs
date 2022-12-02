extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

mod examples;
mod rtg;
mod utils;
mod webgl;
use crate::examples::colored_square::main as draw_colored_square;
use crate::examples::colored_square_rotate::start as start_colored_square_rotate;
use crate::webgl::get_context_by_id;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn render(_vertex_shader: &str, _fragment_shader: &str) -> Result<(), JsValue> {
    let (context, canvas) = get_context_by_id("canvas").unwrap();

    draw_colored_square(&context, canvas.height, canvas.width)?;

    Ok(())
}

#[wasm_bindgen]
pub fn render_rotating_colored_square() -> Result<(), JsValue> {
    // let (context, canvas) = get_context_by_id("canvas").unwrap();

    // start_colored_square_rotate(&context, canvas.height, canvas.width)?;
    start_colored_square_rotate()?;

    Ok(())
}
