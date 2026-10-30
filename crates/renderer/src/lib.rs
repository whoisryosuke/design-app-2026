//! "Hello world" for Vello GPU through wgpu's WebGPU backend in the browser via WASM (phew!)
//!
//! Creates a canvas, builds a simple scene, and renders it once.

#![cfg(target_arch = "wasm32")]

use wasm_bindgen::prelude::*;

mod app_state;
mod design;
mod renderer;

/// This runs automatically when WASM is initialized.
/// Currently used for debugging.
#[wasm_bindgen(start)]
pub fn main_js() {
    // This runs automatically on WASM load
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = requestAnimationFrame)]
    fn request_animation_frame(f: &Closure<dyn FnMut(f64)>);
}
