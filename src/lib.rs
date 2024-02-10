use core::f64;
use std::cell::RefCell;
use std::rc::Rc;

mod bus;
mod cpu;
mod display;
mod rom;

use bus::Bus;
use cpu::Cpu;

cfg_if::cfg_if! {
    if #[cfg(feature = "wasm")] {
        extern crate js_sys;
        extern crate wasm_bindgen;
        extern crate web_sys;

        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;
        use web_sys::CanvasRenderingContext2d;

        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(js_namespace = console)]
            fn log(s: &str);

            #[wasm_bindgen(js_namespace = console)]
            fn error(s: &str);
        }

        fn window() -> web_sys::Window {
            web_sys::window().expect("global `window` should be OK.")
        }

        fn request_animation_frame(f: &Closure<dyn FnMut()>) {
            window()
                .request_animation_frame(f.as_ref().unchecked_ref())
                .expect("`requestAnimationFrame` should be OK.");
        }

        fn cancel_animation_frame(id: i32) {
            window()
                .cancel_animation_frame(id)
                .expect("`cancelAnimationFrame` should be OK.");
        }


        pub struct WasmEmulator {
            pub bus: Bus,
            pub cpu: Cpu,
        }

        pub fn sleep(millis: u64) {
            let start = js_sys::Date::now();
            let mut current = start;
            while current - start < millis as f64 {
                current = js_sys::Date::now();
            }
        }
    }
}

pub const HARD_WIDTH: usize = 800;
pub const HARD_HEIGHT: usize = 600;

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    let document = window().document().expect("window should have a document");

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let rom = include_bytes!("../roms/SPACE_INVADERS").to_vec();
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

    canvas.set_width(HARD_WIDTH as u32);
    canvas.set_height(HARD_HEIGHT as u32);

    let _ = context.scale(10.0, 10.0);

    let max_timeout = 20;
    let mut current_timeout = 0;

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || loop {
        wasm_emulator.step();

        if wasm_emulator.cpu.draw_enable {
            wasm_emulator.draw_graphics(&context);
            request_animation_frame(f.borrow().as_ref().unwrap());
            current_timeout = 0;
            wasm_emulator.cpu.draw_enable = false;
            break;
        }

        if current_timeout >= max_timeout {
            current_timeout = 0;
            break;
        }
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
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
            self.bus.write_memory(0x200 + (i as u16), *item);
        }
    }

    pub fn step(&mut self) {
        self.cpu.fetch_execute(&mut self.bus, None);
    }

    pub fn draw_graphics(&mut self, context: &CanvasRenderingContext2d) {
        let buffer = self.bus.display;

        context.set_fill_style(&JsValue::from_str("black"));
        context.fill_rect(
            0.0,
            0.0,
            (HARD_WIDTH as f64) * 2.0,
            (HARD_HEIGHT as f64) * 2.0,
        );

        context.set_fill_style(&JsValue::from_str("white"));

        // log(format!("{:?}", buffer.as_ref()).as_str());
        for y in 0..32 {
            for x in 0..64 {
                let pixel = buffer[y][x];

                match pixel {
                    display::Color::Black => context.fill_rect(x as f64, y as f64, 1.0, 1.0),
                    _ => {}
                };
            }
        }
    }
}
