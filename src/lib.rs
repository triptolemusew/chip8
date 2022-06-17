#[cfg(feature = "wasm")]
extern crate js_sys;
#[cfg(feature = "wasm")]
extern crate wasm_bindgen;
#[cfg(feature = "wasm")]
extern crate web_sys;

#[cfg(feature = "wasm")]
use js_sys::Math;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "wasm")]
use wasm_bindgen::JsCast;
#[cfg(feature = "wasm")]
use web_sys::CanvasRenderingContext2d;

use std::cell::RefCell;
use std::rc::Rc;

mod bus;
mod constants;
mod cpu;
mod display;
mod memory;
mod rom;

use bus::Bus;
use constants::*;
use cpu::Cpu;
use rom::Rom;

#[cfg(feature = "wasm")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

#[cfg(feature = "wasm")]
fn window() -> web_sys::Window {
    web_sys::window().expect("global `window` should be OK.")
}

#[cfg(feature = "wasm")]
fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("`requestAnimationFrame` should be OK.");
}

#[cfg(feature = "wasm")]
fn cancel_animation_frame(id: i32) {
    window()
        .cancel_animation_frame(id)
        .expect("`cancelAnimationFrame` should be OK.");
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    let document = window().document().expect("window should have a document");
    // let body = document.body().expect("document should have a body");

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let rom = include_bytes!("../roms/TANK").to_vec();
    let mut wasm_emulator = WasmEmulator::new();

    // Load the rom
    wasm_emulator.load_rom(&rom);

    // Creating context
    let canvas = document
        .get_element_by_id("chip8")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    // context.scale(1.5, 1.5).unwrap();
    canvas.set_width(1080);
    canvas.set_height(1080);

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        wasm_emulator.step();
        wasm_emulator.draw_graphics(&context);

        request_animation_frame(f.borrow().as_ref().unwrap());

        // let _ = f.borrow_mut().take();
        // return;
    }) as Box<dyn FnMut()>));

    #[cfg(feature = "wasm")]
    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}

#[cfg(feature = "wasm")]
pub struct WasmEmulator {
    bus: Bus,
    cpu: Cpu,
}

#[cfg(feature = "wasm")]
impl WasmEmulator {
    pub fn new() -> Self {
        WasmEmulator {
            bus: Bus::new(),
            cpu: Cpu::new(),
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        for (i, item) in rom.iter().enumerate() {
            self.bus.ram_write_byte(PROGRAM_START + (i as u16), *item);
        }
    }

    pub fn step(&mut self) {
        self.cpu.fetch_execute(&mut self.bus);
    }

    pub fn draw_graphics(&mut self, context: &CanvasRenderingContext2d) {
        let buffer = self.bus.get_display_buffer();
        update_canvas(
            context,
            (64 as f64) * 2.0,
            (32 as f64) * 2.0,
            &buffer,
        );
    }
}

pub fn update_canvas(
    context: &CanvasRenderingContext2d,
    width: f64,
    height: f64,
    buffer: &display::Display,
) {
    context.set_fill_style(&JsValue::from_str("black"));
    context.fill_rect(0.0, 0.0, 1920 as f64, 1080 as f64);

    // For pixel in screen
    context.set_fill_style(&JsValue::from_str("white"));

    for y in 0..1920 {
        for x in 0..1080 {
            if y % 2 == 0 {
                context.fill_rect(x as f64, y as f64, 1.0, 1.0);
            }
        }
    }
}
