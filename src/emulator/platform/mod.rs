use super::machine::*;

#[cfg(feature = "sdl")]
mod native;

#[cfg(feature = "sdl")]
pub use native::NativePlatform;

#[cfg(feature = "wasm")]
pub mod wasm;

pub trait Platform {
    fn init(&mut self);
    fn listen_for_input(&mut self) -> bool;
    fn get_key_state(&self) -> [bool; NUM_KEYS];
    fn draw_graphics(&mut self, buffer: display::Display);
    fn sleep(&self, duration: u64);
}
