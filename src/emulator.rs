use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::bus::Bus;
use crate::constants::*;
use crate::cpu::Cpu;
use crate::graphics::Graphics;
use crate::rom::Rom;
use crate::display::DisplaySink;

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

    // pub fn step(&mut self) {
    //     self.cpu.fetch_execute(&mut self.bus);
    // }

    pub fn run(&mut self) {
        let mut graphics = Graphics::new(&self.sdl, 800, 600);
        let mut event = self.sdl.event_pump().unwrap();

        'running: loop {
            let mut display_sink = DisplaySink::new();
            self.cpu.fetch_execute(&mut self.bus, &mut display_sink);

            // Only render the frame when it's available as a full buffer
            if let Some(buffer) = display_sink.consume() {
                graphics.draw(buffer.as_ref());
            }

            // TODO: Refactor this out to a gamepad to read keypresses
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
}
