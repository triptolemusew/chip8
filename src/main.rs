extern crate sdl2;

mod memory;
mod rom;
mod constants;
mod graphics;

use rom::Rom;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_name = match args.len() {
        _ => args.get(1).unwrap()
    };

    let rom = Rom::new(&String::from(file_name));
    println!("rom contents: {:?}", rom.contents);
}
