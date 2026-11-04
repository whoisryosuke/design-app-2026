//! "Hello world" for Vello GPU through wgpu's WebGPU backend in the browser via WASM (phew!)
//!
//! Creates a canvas, builds a simple scene, and renders it once.

use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
mod app_state;
mod command_queue;
mod design;
#[cfg(target_arch = "wasm32")]
mod engine;
#[cfg(target_arch = "wasm32")]
mod renderer;
mod utils;

/// This runs automatically when WASM is initialized.
/// Currently used for debugging.
#[wasm_bindgen(start)]
pub fn main_js() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = requestAnimationFrame)]
    fn request_animation_frame(f: &Closure<dyn FnMut(f64)>);
}

#[cfg(test)]
mod tests {
    #[test]
    fn force_ts_generation() {
        assert!(true);
    }
}
