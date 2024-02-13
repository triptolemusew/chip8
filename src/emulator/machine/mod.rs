mod bus;
mod cpu;
pub mod display;
pub mod keypad;

use std::time::{Duration, Instant};

use self::bus::Bus;
use self::cpu::Cpu;
use self::display::DisplaySink;

use super::games::ROMS;
use super::platform::Platform;

pub use self::keypad::NUM_KEYS;

pub struct Emulator {
    context: Box<dyn Platform>,
    pub bus: bus::Bus,
    pub cpu: cpu::Cpu,
    pub current_game: Option<String>,
}

impl Emulator {
    pub fn new(context: Box<dyn Platform>) -> Self {
        let mut ret = Self {
            context,
            bus: Bus::new(),
            cpu: Cpu::new(),
            current_game: Some("TANK".to_string()),
        };
        ret.context.init();
        ret
    }

    fn reset(&mut self) {
        self.cpu.reset();
        self.bus.reset();
    }

    pub fn draw_graphics(&mut self, buffer: display::Display) {
        self.context.draw_graphics(buffer);
    }

    pub fn step(&mut self) {
        self.cpu.step(&mut self.bus);
    }

    pub fn update_keys(&mut self) {
        self.set_keys(self.context.get_key_state());
    }

    pub fn set_keys(&mut self, keys: [bool; NUM_KEYS]) {
        self.cpu.set_keys(keys);
    }

    pub fn load_game(&mut self, name: &str) -> Result<usize, ()> {
        self.reset();

        if let Some(rom) = ROMS.get(name) {
            self.current_game = Some(name.to_string());

            for (idx, &byte) in rom.iter().enumerate() {
                self.bus.write_memory(0x200 + (idx as u16), byte);
            }
            let num_bytes = rom.len();
            Ok(num_bytes)
        } else {
            Err(())
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        for (i, item) in rom.iter().enumerate() {
            self.bus.write_memory(0x200 + (i as u16), *item);
        }
    }

    pub fn run(&mut self) {
        let frame_time = Duration::from_millis(500 / 30);

        'main: loop {
            let mut display_sink = DisplaySink::new();
            let start_time = Instant::now();

            for _ in 0..10 {
                // self.cpu.step(&mut self.bus, &mut display_sink);
                self.cpu.step(&mut self.bus);
            }

            if self.cpu.draw_enable {
                display_sink.append(self.bus.display.clone());
                if let Some(buffer) = display_sink.consume() {
                    self.context.draw_graphics(buffer);
                }
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
