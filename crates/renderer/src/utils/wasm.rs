use serde_wasm_bindgen::Serializer;
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

pub fn create_serializer() -> Serializer {
    let serializer = Serializer::new().serialize_large_number_types_as_bigints(true);
    return serializer;
}
