use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::bus::Bus;
use crate::constants::*;
use crate::cpu::Cpu;
use crate::graphics::Graphics;
use crate::rom::Rom;

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
        let mut event = self.sdl.event_pump().unwrap();

        'running: loop {
            self.run_instruction();

            let buffer = self.bus.get_display_buffer();
            graphics.draw(buffer.as_ref());

            for event in event.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }
        }
    }

    fn run_instruction(&mut self) {
        self.cpu.fetch_execute(&mut self.bus);
    }
}
