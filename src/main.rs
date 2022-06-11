extern crate sdl2;

mod rom;
mod bus;
mod constants;
mod graphics;
mod display;
mod memory;

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
