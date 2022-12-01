use wasm_bindgen::JsValue;
use web_sys::console::log_1;

pub fn log(s: &str) {
    log_1(&JsValue::from(s));
}
