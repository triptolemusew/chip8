extern crate sdl2;

mod rom;
mod bus;
mod constants;
mod graphics;
mod display;
mod memory;
mod cpu;
mod emulator;

mod lib;

use emulator::Emulator;
use rom::Rom;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_name = match args.len() {
        _ => args.get(1).unwrap()
    };

    let rom = Rom::new(&String::from(file_name));

    let mut emulator = Emulator::new();

    emulator.load_rom(&rom);
    emulator.run();
}
