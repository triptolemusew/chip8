use crate::ROMS;

use super::*;
use console_error_panic_hook::set_once;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, Document, Window};

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

pub fn update_canvas(
    context: &CanvasRenderingContext2d,
    width: f64,
    height: f64,
    buffer: display::Display,
) -> Result<()> {
    context.set_fill_style(&JsValue::from_str("black"));
    context.fill_rect(0.0, 0.0, width, height);
    context.set_fill_style(&JsValue::from_str("white"));

    for y in 0..32 {
        for x in 0..64 {
            let pixel = buffer[y][x];
            match pixel {
                display::Color::Black => context.fill_rect(x as f64, y as f64, 1.0, 1.0),
                _ => {}
            }
        }
    }

    Ok(())
}

fn attach_game_listener(document: &Document) -> Result<()> {
    update_all()?;

    let callback = Closure::wrap(Box::new(move |_evt: web_sys::Event| {
        update_all().expect("could not update");
    }) as Box<dyn Fn(_)>);

    document
        .get_element_by_id("game-select")
        .unwrap()
        .dyn_into::<web_sys::HtmlSelectElement>()?
        .set_onchange(Some(callback.as_ref().unchecked_ref()));

    callback.forget();

    Ok(())
}
fn attach_keydown_listener(document: &Document) -> Result<()> {
    let callback = Closure::wrap(Box::new(move |evt: web_sys::Event| {
        let evt = evt.dyn_into::<web_sys::KeyboardEvent>().unwrap();
        let c = std::char::from_u32(evt.key_code()).unwrap();
        if let Ok(ch) = keyboard_to_keypad(c) {
            KEYS.key_down(ch);
        } else if c == 'G' {
            *TRIGGER_RESTART.write().unwrap() = true;
        }
    }) as Box<dyn FnMut(_)>);

    document.add_event_listener_with_callback("keydown", callback.as_ref().unchecked_ref())?;

    callback.forget();
    Ok(())
}

fn attach_keyup_listener(document: &Document) -> Result<()> {
    let callback = Closure::wrap(Box::new(move |evt: web_sys::Event| {
        let evt = evt.dyn_into::<web_sys::KeyboardEvent>().unwrap();
        let c = std::char::from_u32(evt.key_code()).unwrap();
        if let Ok(ch) = keyboard_to_keypad(c) {
            KEYS.key_up(ch);
        }
    }) as Box<dyn FnMut(_)>);

    document.add_event_listener_with_callback("keyup", callback.as_ref().unchecked_ref())?;

    callback.forget();
    Ok(())
}

fn update_all() -> Result<()> {
    let document = get_document();
    let new_game_select = document
        .get_element_by_id("game-select")
        .unwrap()
        .dyn_into::<web_sys::HtmlSelectElement>()?;

    *CURRENT_GAME.write().unwrap() = new_game_select.value().to_string();
    Ok(())
}

fn list_all_roms(document: &Document) -> Result<()> {
    let select = document
        .get_element_by_id("game-select")
        .unwrap()
        .dyn_into::<web_sys::HtmlSelectElement>()?;

    for rom in ROMS.keys() {
        let selected = rom == &*CURRENT_GAME.read().unwrap();
        let new_option =
            web_sys::HtmlOptionElement::new_with_text_and_value_and_default_selected_and_selected(
                rom, rom, selected, selected,
            )?;
        select.append_child(&new_option)?;
    }

    Ok(())
}

pub fn mount() {
    set_once();
    let document = get_document();
    // Append all of the available roms
    list_all_roms(&document).unwrap();
    // Attach DOM event listener
    attach_game_listener(&document).unwrap();
    attach_keydown_listener(&document).unwrap();
    attach_keyup_listener(&document).unwrap();
}
