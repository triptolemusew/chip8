use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::keyboard::{KeyboardState, Keycode, PressedScancodeIterator, Scancode};

use crate::bus::Bus;
use crate::cpu::Cpu;
use crate::display::DisplaySink;
use crate::graphics::Graphics;
use crate::rom::Rom;

// Constants
pub const PROGRAM_START: u16 = 0x200;
pub const CLOCK_SPEED: u64 = 500;
pub const REFRESH_RATE: u64 = 60;

// pub const CYCLES_PER_SECOND: u64 = 500;
// pub const CYCLES_PER_SLEEP: u64 = 10;
// pub const MILLIS_PER_SLEEP: f64 = (CYCLES_PER_SLEEP as f64 / CYCLES_PER_SECOND as f64) * 1000.0;

pub struct Emulator {
    bus: Bus,
    cpu: Cpu,
    sdl: sdl2::Sdl,
}

impl Emulator {
    pub fn new() -> Self {
        Emulator {
            bus: Bus::new(),
            cpu: Cpu::new(),
            sdl: sdl2::init().unwrap(),
        }
    }

    pub fn load_rom(&mut self, rom: &Rom) {
        for i in 0..rom.get_rom_size() {
            self.bus
                .ram_write_byte(PROGRAM_START + (i as u16), rom.contents[i]);
        }
    }

    pub fn run(&mut self) {
        let mut graphics = Graphics::new(&self.sdl, 800, 600);
        let mut timer = self.sdl.timer().unwrap();
        let mut events = self.sdl.event_pump().unwrap();

        'main: loop {
            let start_time = Instant::now();
            let frame_time = Duration::from_millis(500 / 60);

            let mut display_sink = DisplaySink::new();
            self.cpu.fetch_execute(&mut self.bus, &mut display_sink);

            // Only render the frame when it's available as a full buffer
            if let Some(buffer) = display_sink.consume() {
                graphics.draw(buffer.as_ref());
            }

            // TODO: Refactor this out to a gamepad to read keypresses
            for event in events.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'main,
                    _ => {}
                }
            }

            for key in events.keyboard_state().pressed_scancodes().into_iter() {
                let pressed = match key {
                    Scancode::Num0 => 0x0,
                    Scancode::Num1 => 0x1,
                    Scancode::Num2 => 0x2,
                    Scancode::Num3 => 0x3,
                    Scancode::Num4 => 0x4,
                    Scancode::Num5 => 0x5,
                    Scancode::Num6 => 0x6,
                    Scancode::Num7 => 0x7,
                    Scancode::Num8 => 0x8,
                    Scancode::Num9 => 0x9,
                    Scancode::A => 0xA,
                    Scancode::B => 0xB,
                    Scancode::C => 0xC,
                    Scancode::D => 0xD,
                    Scancode::E => 0xE,
                    Scancode::F => 0xF,
                    _ => -1,
                };

                if pressed >= 0 {
                    self.cpu.keypad[pressed as usize] = true;
                }
            }

            let elapsed_time = start_time.elapsed();
            if elapsed_time < frame_time {
                let remaining_time = frame_time - elapsed_time;
                std::thread::sleep(remaining_time);
            }
        }
    }
}
