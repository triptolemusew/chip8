mod dom;

use std::sync::{Arc, RwLock};

use self::dom::{get_document, update_canvas};

use super::*;
use js_sys::Math::{floor, random};
use lazy_static::lazy_static;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;

#[derive(Debug)]
pub struct WasmContext {
    ctx: Option<CanvasRenderingContext2d>,
    width: u32,
    height: u32,
}

impl WasmContext {
    pub fn new(width: u32, height: u32) -> Box<Self> {
        Box::new(Self {
            width,
            height,
            ctx: None,
        })
    }
}

lazy_static! {
    static ref KEYS: Keys = Keys::new(); // TODO I think the Arc should jus tbe here, dont make Machine worry about it
    static ref CURRENT_GAME: Arc<RwLock<String>> = Arc::new(RwLock::new("test_opcode".to_string()));
    static ref TRIGGER_RESTART: Arc<RwLock<bool>> = Arc::new(RwLock::new(false));
}

impl Context for WasmContext {
    fn init(&mut self) {
        // Mount the DOM
        mount();

        let document = get_document();

        let canvas = document
            .get_element_by_id("chip8")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();

        canvas.set_width(self.width);
        canvas.set_height(self.height);

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        self.ctx = Some(context);
    }

    fn listen_for_input(&mut self) -> bool {
        false
    }

    fn draw_graphics(&mut self, buffer: &[u8]) {
        update_canvas(
            self.ctx.as_ref().unwrap(),
            self.width as f64,
            self.height as f64,
        )
        .unwrap();
    }
}

#[wasm_bindgen]
pub fn run() {
    let context = WasmContext::new(, height);
    let emulator = Emulator::new(context);

    let default_game = &*CURRENT_GAME.read().unwrap();
    let bytes = emulator.load_rom(rom).unwrap();

    let f = Rc::new(RefCell::new(None));
}
