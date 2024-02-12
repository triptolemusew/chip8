mod bus;
mod cpu;
mod display;

use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use crate::rom::Rom;

use self::bus::Bus;
use self::cpu::Cpu;
use self::display::DisplaySink;

use super::context::Context;

pub const NUM_KEYS: usize = 16;
pub const SCREEN_HEIGHT: usize = 32;
pub const SCREEN_WIDTH: usize = 64;

pub struct Keys {
    state: Arc<RwLock<[bool; NUM_KEYS]>>,
}

impl Default for Keys {
    fn default() -> Self {
        Self {
            state: Arc::new(RwLock::new([false; NUM_KEYS])),
        }
    }
}

impl Keys {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn key_down(&self, key: u8) {
        self.state.write().unwrap()[key as usize] = true;
    }

    pub fn key_up(&self, key: u8) {
        self.state.write().unwrap()[key as usize] = false;
    }

    pub fn is_pressed(&self, key: u8) -> bool {
        let key = key as usize;
        if key >= NUM_KEYS {
            false
        } else {
            self.state.read().unwrap()[key]
        }
    }

    pub fn inner(&self) -> [bool; NUM_KEYS] {
        *self.state.read().unwrap()
    }
}

pub fn keyboard_to_keypad(keyboard: char) -> Result<u8, ()> {
    match keyboard.to_ascii_uppercase() {
        '1' => Ok(1),
        '2' => Ok(2),
        '3' => Ok(3),
        '4' => Ok(0xC),
        'Q' => Ok(4),
        'W' => Ok(5),
        'E' => Ok(6),
        'R' => Ok(0xD),
        'A' => Ok(7),
        'S' => Ok(8),
        'D' => Ok(9),
        'F' => Ok(0xE),
        'Z' => Ok(0xA),
        'X' => Ok(0),
        'C' => Ok(0xB),
        'V' => Ok(0xF),
        _ => Ok(0),
    }
}

pub struct Emulator {
    context: Box<dyn Context>,
    bus: bus::Bus,
    cpu: cpu::Cpu,
}

impl Emulator {
    pub fn new(context: Box<dyn Context>) -> Self {
        Emulator {
            context,
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
        let frame_time = Duration::from_millis(500 / 30);

        'main: loop {
            let mut display_sink = DisplaySink::new();
            let start_time = Instant::now();

            for _ in 0..10 {
                self.cpu.step(&mut self.bus, &mut display_sink);
            }

            if let Some(buffer) = display_sink.consume() {
                self.context.draw_graphics(buffer.as_ref());
            }

            if self.context.listen_for_input() {
                // Handle keypresses
                break 'main;
            }

            self.cpu.set_keys(self.context.get_key_state());

            let end_time: Instant = Instant::now();
            if end_time - start_time < frame_time {
                let sleep_duration = frame_time - (end_time - start_time);
                self.context.sleep(sleep_duration.as_millis() as u64);
            }
        }
    }
}
