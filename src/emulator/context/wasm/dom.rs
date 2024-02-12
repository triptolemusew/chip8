use super::*;
use console_error_panic_hook::set_once;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, Document, Element, Window};

pub fn sleep(duration: u64) {
    let start = js_sys::Date::now();
    let mut current = start;
    while current - start < duration as f64 {
        current = js_sys::Date::now();
    }
}

fn window() -> Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn get_document() -> Document {
    window()
        .document()
        .expect("should have a document on the window")
}

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("`requestAnimationFrame` should be OK.");
}

pub fn update_canvas(context: &CanvasRenderingContext2d, width: f64, height: f64) {
    context.set_fill_style(&JsValue::from_str("black"));
    context.fill_rect(0.0, 0.0, width, height);
    context.set_fill_style(&JsValue::from_str("white"));

    for y in 0..32 {
        for x in 0..64 {
            context.fill_rect(x as f64, y as f64, 1.0, 1.0);
        }
    }
}

fn attach_game_listener(document: &Document) -> Result<()> {
    update_all()?;

    let callback = Closure::wrap(Box::new(move |_evt: web_sys::Event| {
        update_all().expect("could not update");
    }) as Box<dyn Fn(_)>);

}
