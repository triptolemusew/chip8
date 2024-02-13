pub mod emulator;
pub mod cartridge;

pub use emulator::games::ROMS;

#[cfg(feature = "sdl")]
pub use emulator::NativePlatform;

pub const APP_NAME: &str = "chip8";

pub const APP_WIDTH: u32 = 64;
pub const APP_HEIGHT: u32 = 32;
pub const APP_SCALE_FACTOR: u32 = 10;
