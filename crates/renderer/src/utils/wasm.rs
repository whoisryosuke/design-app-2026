use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, window};

pub fn get_canvas_element(canvas_id: &str) -> Option<HtmlCanvasElement> {
    // @TODO: Maybe pass Result + error?
    window()?
        .document()?
        .get_element_by_id(canvas_id)?
        .dyn_into::<HtmlCanvasElement>()
        .ok()
}
