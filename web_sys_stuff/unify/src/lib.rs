#[path = "mod.rs"]
pub mod r#mod;
pub use r#mod::*;

// First up let's take a look of binding `console.log` manually, without the
// help of `web_sys`. Here we're writing the `#[wasm_bindgen]` annotations
// manually ourselves, and the correctness of our program relies on the
// correctness of these annotations!

#[wasm_bindgen(start)]
fn main() {
    bare_bones();
    using_a_macro();
    using_web_sys();
    start_canvas();
    run_closures();
    run_dom();
    // run_fetch("CoAuToSe/cargo-cats".to_string());
    start_paint();
    run_performance();
    run_request_animation_frame();
    run_wasm_wasm();
    run_wasm_wasm_imports();
    run_weather();
    start_webgl();
    start_websocket();
}
