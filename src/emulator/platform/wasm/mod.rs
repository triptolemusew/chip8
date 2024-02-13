mod dom;
mod js_interop;

use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, RwLock},
    time::Duration,
};

use self::dom::*;
use super::display::DisplaySink;
use super::keypad::Keys;
use super::*;
use crate::{APP_HEIGHT, APP_SCALE_FACTOR, APP_WIDTH};

use lazy_static::lazy_static;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;

pub type Result<T> = std::result::Result<T, JsValue>;

#[derive(Debug)]
pub struct WasmPlatform {
    ctx: Option<CanvasRenderingContext2d>,
    width: u32,
    height: u32,
}

lazy_static! {
    static ref KEYS: Keys = Keys::new();
    static ref CURRENT_GAME: Arc<RwLock<String>> = Arc::new(RwLock::new("TANK".to_string()));
    static ref TRIGGER_RESTART: Arc<RwLock<bool>> = Arc::new(RwLock::new(false));
}

impl WasmPlatform {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            width: APP_WIDTH,
            height: APP_HEIGHT,
            ctx: None,
        })
    }
}

impl Platform for WasmPlatform {
    fn init(&mut self) {
        mount();

        let document = get_document();

        let canvas = document
            .get_element_by_id("chip8")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();

        canvas.set_width(self.width * APP_SCALE_FACTOR);
        canvas.set_height(self.height * APP_SCALE_FACTOR);

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let _ = context.scale(APP_SCALE_FACTOR as f64, APP_SCALE_FACTOR as f64);
        self.ctx = Some(context);
    }

    fn listen_for_input(&mut self) -> bool {
        false
    }

    fn draw_graphics(&mut self, buffer: display::Display) {
        update_canvas(
            self.ctx.as_ref().unwrap(),
            self.width as f64,
            self.height as f64,
            buffer,
        )
        .unwrap();
    }

    fn sleep(&self, duration: u64) {
        sleep(duration);
    }

    fn get_key_state(&self) -> [bool; NUM_KEYS] {
        KEYS.inner()
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn run() -> Result<()> {
    let context = WasmPlatform::new();
    let mut emulator = Emulator::new(context);
    let default_game = &*CURRENT_GAME.read().unwrap();
    emulator.load_game(default_game).unwrap();

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let frame_time = Duration::from_millis(500 / 30).as_secs_f64();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let start_time = js_sys::Date::now();
        let selected_game = &*CURRENT_GAME.read().unwrap();
        let mut display_sink = DisplaySink::new();

        if let Some(name) = &emulator.current_game {
            if name != selected_game || *TRIGGER_RESTART.read().unwrap() {
                *TRIGGER_RESTART.write().unwrap() = false;
                let _ = emulator
                    .load_game(&selected_game)
                    .expect("could not load new rom");
            }
        }

        loop {
            emulator.step();
            emulator.update_keys();

            if emulator.cpu.draw_enable {
                display_sink.append(emulator.bus.display.clone());
                if let Some(buffer) = display_sink.consume() {
                    emulator.draw_graphics(buffer);
                }
                request_animation_frame(f.borrow().as_ref().unwrap());
                emulator.cpu.draw_enable = false;
                break;
            }

            let end_time = js_sys::Date::now();
            if end_time - start_time < frame_time {
                sleep(u64::from(
                    frame_time as u64 - (end_time as u64 - start_time as u64),
                ));
            }
        }
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}
