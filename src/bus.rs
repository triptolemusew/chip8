use crate::memory::Memory;
use crate::display::Display;

pub struct Bus {
    memory: Memory,
    display: Display,
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            memory: Memory::new(),
            display: Display::new(),
        }
    }

    pub fn ram_read_byte(&self, address: u16) -> u8 {
        self.memory.read_byte(address)
    }

    pub fn ram_write_byte(&mut self, address: u16, value: u8) {
        self.memory.write_byte(address, value)
    }

    pub fn get_display_buffer(&mut self) -> &mut Display {
        &mut self.display
    }

    pub fn clear_screen(&mut self) {
        self.display.clear();
    }
}
