use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, Window};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[allow(dead_code)]
pub fn console_log(log: &str) {
    let array = js_sys::Array::new();
    array.push(&JsValue::from_str(log));
    web_sys::console::log(&array);
}

pub struct Browser {
}

impl Browser {
    pub fn window() -> Window {
        web_sys::window().expect("no global `window` exists")
    }

    pub fn document() -> Document {
        Self::window()
            .document()
            .expect("should have a document on window")
    }

    pub fn canvas(id: &str) -> HtmlCanvasElement {
        Self::document()
            .query_selector(id)
            .unwrap()
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .expect("No canvas found.")
    }

    pub fn context(canvas: &HtmlCanvasElement) -> CanvasRenderingContext2d {
        canvas
            .get_context("2d")
            .expect("No context created.")
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .expect("Could not be fetched as internal object.")
    }

    pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
        Self::window()
            .request_animation_frame(f.as_ref().unchecked_ref())
            .expect("should register `requestAnimationFrame` OK");
    }
}
