use std::sync::{Arc, RwLock};

pub const NUM_KEYS: usize = 16;

pub struct Keys {
    pub state: Arc<RwLock<[bool; NUM_KEYS]>>,
}

impl Default for Keys {
    fn default() -> Self {
        Self {
            state: Arc::new(RwLock::new([false; NUM_KEYS])),
        }
    }
}

impl Keys {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn key_down(&self, key: u8) {
        self.state.write().unwrap()[key as usize] = true;
    }

    pub fn key_up(&self, key: u8) {
        self.state.write().unwrap()[key as usize] = false;
    }

    pub fn is_pressed(&self, key: u8) -> bool {
        let key = key as usize;
        if key >= NUM_KEYS {
            false
        } else {
            self.state.read().unwrap()[key]
        }
    }

    pub fn inner(&self) -> [bool; NUM_KEYS] {
        *self.state.read().unwrap()
    }
}

pub fn keyboard_to_keypad(keyboard: char) -> Result<u8, ()> {
    match keyboard.to_ascii_uppercase() {
        '1' => Ok(1),
        '2' => Ok(2),
        '3' => Ok(3),
        '4' => Ok(0xC),
        'Q' => Ok(4),
        'W' => Ok(5),
        'E' => Ok(6),
        'R' => Ok(0xD),
        'A' => Ok(7),
        'S' => Ok(8),
        'D' => Ok(9),
        'F' => Ok(0xE),
        'Z' => Ok(0xA),
        'X' => Ok(0),
        'C' => Ok(0xB),
        'V' => Ok(0xF),
        _ => Ok(0),
    }
}
