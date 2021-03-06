use crate::display::Display;

const FONTS: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80, 0xF0, 0xF0,
    0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0, 0xF0, 0x80,
    0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0,
    0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xe0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80,
    0x80, 0xF0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
];

pub struct Bus {
    pub memory: Vec<u8>,
    pub display: Display,
}

impl Bus {
    pub fn new() -> Self {
        let mut memory = vec![0; 4096];

        memory[..80].clone_from_slice(&FONTS);

        Bus {
            memory,
            display: Display::default(),
        }
    }

    pub fn read_memory(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write_memory(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
}
