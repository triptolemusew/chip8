use crate::{APP_HEIGHT, APP_WIDTH};
use std::ops::{Index, IndexMut};

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Color {
    Black = 0x00,
    White = 0xFF,
}

#[derive(Clone, Copy)]
pub struct Display {
    pixels: [Color; (APP_HEIGHT * APP_WIDTH) as usize],
}

impl Display {
    #[allow(unused)]
    pub fn get_index_from_coords(x: usize, y: usize) -> usize {
        y * APP_WIDTH as usize + x
    }

    pub fn clear(&mut self) {
        for pixel in self.pixels.iter_mut() {
            *pixel = Color::Black;
        }
    }
}

impl Default for Display {
    fn default() -> Self {
        Self {
            pixels: [Color::Black; (APP_WIDTH * APP_HEIGHT) as usize],
        }
    }
}

impl AsRef<[u8]> for Display {
    fn as_ref(&self) -> &[u8] {
        unsafe { &*(&self.pixels as *const [Color] as *const [u8]) }
    }
}

impl Index<usize> for Display {
    type Output = [Color];
    fn index(&self, index: usize) -> &Self::Output {
        let start = index * APP_WIDTH as usize;
        &self.pixels[start..(start + APP_WIDTH as usize)]
    }
}

impl IndexMut<usize> for Display {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let start = index * APP_WIDTH as usize;
        &mut self.pixels[start..(start + APP_WIDTH as usize)]
    }
}

pub struct DisplaySink {
    inner: Option<Display>,
}

impl DisplaySink {
    pub fn new() -> Self {
        DisplaySink { inner: None }
    }

    pub fn consume(self) -> Option<Display> {
        self.inner
    }

    pub fn append(&mut self, value: Display) {
        self.inner = Some(value);
    }
}
