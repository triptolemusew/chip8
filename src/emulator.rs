use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::keyboard::{KeyboardState, Keycode, Scancode};

use crate::bus::Bus;
use crate::cpu::Cpu;
use crate::display::DisplaySink;
use crate::rom::Rom;
use crate::texture::SdlTexture;

pub struct Emulator {
    bus: Bus,
    cpu: Cpu,
}

impl Emulator {
    pub fn new() -> Self {
        Emulator {
            bus: Bus::new(),
            cpu: Cpu::new(),
        }
    }

    pub fn load_rom(&mut self, rom: &Rom) {
        for (i, item) in rom.contents.iter().enumerate() {
            self.bus.write_memory(0x200 + (i as u16), *item);
        }
    }

    pub fn run(&mut self) {
        let sdl = sdl2::init().unwrap();
        let frame_time = Duration::from_millis(500 / 30);

        let mut texture = SdlTexture::new(&sdl, 640, 320);
        let mut events = sdl.event_pump().unwrap();

        'main: loop {
            let mut display_sink = DisplaySink::new();
            let start_time = Instant::now();
            for _ in 0..8 {
                self.cpu.fetch_execute(&mut self.bus, Some(&mut display_sink));
            }

            // Only render the frame when it's available as a full buffer
            if let Some(buffer) = display_sink.consume() {
                texture.draw(buffer.as_ref());
            }

            for event in events.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        repeat: false,
                        ..
                    } => break 'main,
                    _ => {}
                }
            }

            // Keypad stuff
            let key_state = KeyboardState::new(&mut events);
            for i in 0..=0xF {
                self.cpu.keypad[i as usize] = key_state.is_scancode_pressed(get_sdl_keycode(i));
            }

            let end_time: Instant = Instant::now();
            if end_time - start_time < frame_time {
                std::thread::sleep(frame_time - (end_time - start_time));
            }
        }
    }
}

fn get_sdl_keycode(key: usize) -> Scancode {
    match key {
        0x0 => Scancode::Num0,
        0x1 => Scancode::Num1,
        0x2 => Scancode::Num2,
        0x3 => Scancode::Num3,
        0x4 => Scancode::Num4,
        0x5 => Scancode::Num5,
        0x6 => Scancode::Num6,
        0x7 => Scancode::Num7,
        0x8 => Scancode::Num8,
        0x9 => Scancode::Num9,
        0xA => Scancode::A,
        0xB => Scancode::B,
        0xC => Scancode::C,
        0xD => Scancode::D,
        0xE => Scancode::E,
        0xF => Scancode::F,
        _ => panic!(),
    }
}
