use std::{fs::*, io::Read};

#[derive(Debug)]

pub struct Rom {
    pub contents: Vec<u8>,
}

impl Rom {
    pub fn new(path: &String) -> Self {
        let mut f = File::open(&path).expect("no file found");
        let metadata = std::fs::metadata(&path).expect("unable to load metadata");

        let mut contents = vec![0; metadata.len() as usize];

        f.read(&mut contents).expect("overflow");

        Rom { contents }
    }

    pub fn get_rom_size(&self) -> usize {
        self.contents.len()
    }
}
