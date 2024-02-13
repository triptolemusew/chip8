mod platform;
mod machine;
pub mod games;

pub use machine::Emulator;

#[cfg(feature = "sdl")]
pub use platform::NativePlatform;

#[cfg(feature = "wasm")]
pub use platform::wasm::run;
