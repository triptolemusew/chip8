pub const SCREEN_HEIGHT: usize = 32;
pub const SCREEN_WIDTH: usize = 64;

pub const PROGRAM_START: u16 = 0x200;

pub const CYCLES_PER_SECOND: u64 = 500;
pub const CYCLES_PER_SLEEP: u64 = 10;
pub const MILLIS_PER_SLEEP: f64 = (CYCLES_PER_SLEEP as f64 / CYCLES_PER_SECOND as f64) * 1000.0;
