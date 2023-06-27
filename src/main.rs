use wasm_bindgen::JsValue;

#[cfg(feat)]
fn f() {
    web_sys::console::log_1(JsValue::from_str("cfg(feat)"));
}

#[cfg(not(feat))]
fn f() {
    web_sys::console::log_1(&JsValue::from_str("cfg(not(feat))"));
}

fn main() {
    f();
}
