// mod bus;
// mod cpu;
// mod display;
pub mod emulator;
pub mod rom;
// mod texture;

#[cfg(feature = "sdl")]
pub use emulator::SdlContext;

// #[cfg(feature = "wasm")]
// pub use emulator::wasm::run;

// use core::f64;
// use std::cell::RefCell;
// use std::rc::Rc;
// use std::time::Duration;
//
// mod bus;
// mod cpu;
// mod display;
// mod rom;
//
// use bus::Bus;
// use cpu::Cpu;
//
// cfg_if::cfg_if! {
//     if #[cfg(feature = "wasm")] {
//         extern crate js_sys;
//         extern crate wasm_bindgen;
//         extern crate web_sys;
//
//         use wasm_bindgen::prelude::*;
//         use wasm_bindgen::JsCast;
//         use web_sys::CanvasRenderingContext2d;
//
//         #[wasm_bindgen]
//         extern "C" {
//             #[wasm_bindgen(js_namespace = console)]
//             fn log(s: &str);
//
//             #[wasm_bindgen(js_namespace = console)]
//             fn error(s: &str);
//         }
//
//         fn window() -> web_sys::Window {
//             web_sys::window().expect("global `window` should be OK.")
//         }
//
//         fn request_animation_frame(f: &Closure<dyn FnMut()>) {
//             window()
//                 .request_animation_frame(f.as_ref().unchecked_ref())
//                 .expect("`requestAnimationFrame` should be OK.");
//         }
//
//         fn cancel_animation_frame(id: i32) {
//             window()
//                 .cancel_animation_frame(id)
//                 .expect("`cancelAnimationFrame` should be OK.");
//         }
//
//         pub struct WasmEmulator {
//             pub bus: Bus,
//             pub cpu: Cpu,
//         }
//
//         pub fn sleep(millis: f64) {
//             let start = js_sys::Date::now();
//             let mut current = start;
//             while current - start < millis as f64 {
//                 current = js_sys::Date::now();
//             }
//         }
//     }
// }
//
// pub const CANVAS_WIDTH: usize = 64;
// pub const CANVAS_HEIGHT: usize = 32;
//
// #[cfg(feature = "wasm")]
// #[wasm_bindgen]
// pub fn run(rom: &[u8]) -> Result<(), JsValue> {
//     let document = window().document().expect("window should have a document");
//
//     let f = Rc::new(RefCell::new(None));
//     let g = f.clone();
//
//     let mut wasm_emulator = WasmEmulator::new();
//     wasm_emulator.load_rom(rom);
//
//     // Creating context
//     let canvas = document
//         .get_element_by_id("chip8")
//         .unwrap()
//         .dyn_into::<web_sys::HtmlCanvasElement>()
//         .unwrap();
//
//     let context = canvas
//         .get_context("2d")
//         .unwrap()
//         .unwrap()
//         .dyn_into::<web_sys::CanvasRenderingContext2d>()
//         .unwrap();
//
//     canvas.set_width((CANVAS_WIDTH * 10) as u32);
//     canvas.set_height((CANVAS_HEIGHT * 10) as u32);
//
//     let _ = context.scale(10.0, 10.0);
//     let frame_time = Duration::from_millis(500 / 30).as_secs_f64();
//
//     *g.borrow_mut() = Some(Closure::wrap(Box::new(move || loop {
//         let start_time = js_sys::Date::now();
//         wasm_emulator.step();
//
//         if wasm_emulator.cpu.draw_enable {
//             wasm_emulator.draw_graphics(&context);
//             request_animation_frame(f.borrow().as_ref().unwrap());
//             wasm_emulator.cpu.draw_enable = false;
//             break;
//         }
//
//         let end_time = js_sys::Date::now();
//         if end_time - start_time < frame_time {
//             sleep(frame_time - (end_time - start_time));
//         }
//     }) as Box<dyn FnMut()>));
//
//     request_animation_frame(g.borrow().as_ref().unwrap());
//
//     Ok(())
// }
//
// #[cfg(feature = "wasm")]
// impl WasmEmulator {
//     pub fn new() -> Self {
//         WasmEmulator {
//             bus: Bus::new(),
//             cpu: Cpu::new(),
//         }
//     }
//
//     pub fn load_rom(&mut self, rom: &[u8]) {
//         for (i, item) in rom.iter().enumerate() {
//             self.bus.write_memory(0x200 + (i as u16), *item);
//         }
//     }
//
//     pub fn step(&mut self) {
//         self.cpu.fetch_execute(&mut self.bus, None);
//     }
//
//     pub fn draw_graphics(&mut self, context: &CanvasRenderingContext2d) {
//         let buffer = self.bus.display;
//
//         context.set_fill_style(&JsValue::from_str("black"));
//
//         context.fill_rect(0.0, 0.0, CANVAS_WIDTH as f64, CANVAS_HEIGHT as f64);
//
//         context.set_fill_style(&JsValue::from_str("white"));
//
//         for y in 0..32 {
//             for x in 0..64 {
//                 let pixel = buffer[y][x];
//
//                 match pixel {
//                     display::Color::Black => context.fill_rect(x as f64, y as f64, 1.0, 1.0),
//                     _ => {}
//                 };
//             }
//         }
//     }
// }
