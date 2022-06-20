use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use std::ops::{Index, IndexMut};

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Color {
    Black = 0x00,
    White = 0xFF,
}

#[derive(Clone, Copy)]
pub struct Display {
    pixels: [Color; SCREEN_HEIGHT * SCREEN_WIDTH],
}

impl Display {
    #[allow(unused)]
    pub fn get_index_from_coords(x: usize, y: usize) -> usize {
        y * SCREEN_WIDTH + x
    }

    pub fn clear(&mut self) {
        for pixel in self.pixels.iter_mut() {
            *pixel = Color::White;
        }
    }
}

impl Default for Display {
    fn default() -> Self {
        Self {
            pixels: [Color::Black; SCREEN_WIDTH * SCREEN_HEIGHT],
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
        let start = index * SCREEN_WIDTH;
        &self.pixels[start..(start + SCREEN_WIDTH)]
    }
}

impl IndexMut<usize> for Display {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let start = index * SCREEN_WIDTH;
        &mut self.pixels[start..(start + SCREEN_WIDTH)]
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
