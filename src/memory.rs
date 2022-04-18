use crate::constants::{FONTSET, MEMORY_SIZE};

#[derive(Debug, Clone)]
pub struct Memory {
    pub contents: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            contents: vec![0; MEMORY_SIZE],
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.contents[address as usize] = value;
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.contents[address as usize]
    }
}
