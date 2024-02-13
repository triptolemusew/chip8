use chip8::{emulator::Emulator, SdlContext};
use chip8::cartridge::Cartridge;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_name = match args.len() {
        _ => args.get(1).unwrap(),
    };

    let rom = Cartridge::new(&String::from(file_name));

    let context = SdlContext::new(64 * 10, 32 * 10);
    let mut emulator = Emulator::new(context);

    emulator.load_rom(&rom.contents);
    emulator.run();
}
