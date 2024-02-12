use super::machine::*;

#[cfg(feature = "sdl")]
mod sdl;

#[cfg(feature = "wasm")]
pub mod wasm;

#[cfg(feature = "sdl")]
pub use sdl::SdlContext;

pub trait Context {
    fn init(&mut self);
    fn listen_for_input(&mut self) -> bool;
    fn get_key_state(&self) -> [bool; NUM_KEYS];
    fn draw_graphics(&mut self, buffer: &[u8]);
    fn sleep(&self, duration: u64);
}
