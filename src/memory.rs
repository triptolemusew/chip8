#[derive(Debug, Clone)]
pub struct Memory {
    pub contents: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        let mut ram = Memory {
            contents: vec![0; 4096],
        };

        let sprites: [[u8; 5]; 16] = [
            [0xF0, 0x90, 0x90, 0x90, 0xF0],
            [0x20, 0x60, 0x20, 0x20, 0x70],
            [0xF0, 0x10, 0xF0, 0x80, 0xF0],
            [0xF0, 0x10, 0xF0, 0x10, 0xF0],
            [0x90, 0x90, 0xF0, 0x10, 0x10],
            [0xF0, 0x80, 0xF0, 0x10, 0xF0],
            [0xF0, 0x80, 0xF0, 0x90, 0xF0],
            [0xF0, 0x10, 0x20, 0x40, 0x40],
            [0xF0, 0x90, 0xF0, 0x90, 0xF0],
            [0xF0, 0x90, 0xF0, 0x10, 0xF0],
            [0xF0, 0x90, 0xF0, 0x90, 0x90],
            [0xE0, 0x90, 0xE0, 0x90, 0xE0],
            [0xF0, 0x80, 0x80, 0x80, 0xF0],
            [0xE0, 0x90, 0x90, 0x90, 0xE0],
            [0xF0, 0x80, 0xF0, 0x80, 0xF0],
            [0xF0, 0x80, 0xF0, 0x80, 0x80],
        ];

        let mut i = 0;
        for sprite in &sprites {
            for ch in sprite {
                ram.contents[i] = *ch;
                i += 1;
            }
        }
        ram
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.contents[address as usize] = value;
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.contents[address as usize]
    }
}
