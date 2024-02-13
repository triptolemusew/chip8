use chip8::cartridge::Cartridge;
use chip8::{emulator::Emulator, NativePlatform};
use chip8::{APP_HEIGHT, APP_SCALE_FACTOR, APP_WIDTH};

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_name = match args.len() {
        _ => args.get(1).unwrap(),
    };

    let cart = Cartridge::new(&String::from(file_name));

    let context = NativePlatform::new(APP_WIDTH, APP_HEIGHT, APP_SCALE_FACTOR);
    let mut emulator = Emulator::new(context);

    emulator.load_rom(&cart.contents);
    emulator.run();
}
