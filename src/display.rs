use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use std::fmt::Debug;

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
        Self { pixels: [Color::Black; SCREEN_WIDTH * SCREEN_HEIGHT] }
    }
}

