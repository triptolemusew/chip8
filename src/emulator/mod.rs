mod context;
mod machine;

pub use machine::Emulator;

#[cfg(feature = "sdl")]
pub use context::SdlContext;
