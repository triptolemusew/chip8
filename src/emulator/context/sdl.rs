use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::{Texture, WindowCanvas};
use sdl2::EventPump;

use std::time::Duration;

use super::*;

pub struct SdlContext {
    canvas: WindowCanvas,
    event_pump: EventPump,
    texture: Texture,
    key_state: Keys,
}

impl SdlContext {
    pub fn new(width: u32, height: u32) -> Box<Self> {
        let context = sdl2::init().unwrap();
        let video_subsystem = context.video().unwrap();
        let window = video_subsystem
            .window("wasm-chip8", width, height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        let mut canvas = window
            .into_canvas()
            .accelerated()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        let event_pump = context.event_pump().unwrap();

        canvas.set_draw_color(Color::RGB(0, 255, 255));

        let texture = canvas
            .texture_creator()
            .create_texture_target(sdl2::pixels::PixelFormatEnum::RGB332, 64, 32)
            .unwrap();

        Box::new(Self {
            canvas,
            event_pump,
            texture,
            key_state: Keys::new(),
        })
    }
}

impl Context for SdlContext {
    fn init(&mut self) {
        self.canvas.clear();
        self.canvas.present();
    }

    fn listen_for_input(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    repeat: false,
                    ..
                } => return true,
                Event::KeyDown { keycode, .. } => {
                    if let Some(key) = keycode_to_keypad(keycode) {
                        self.key_state.key_down(key);
                    }
                }
                Event::KeyUp { keycode, .. } => {
                    if let Some(key) = keycode_to_keypad(keycode) {
                        self.key_state.key_up(key);
                    }
                }
                _ => {}
            }
        }
        false
    }

    fn draw_graphics(&mut self, buffer: &[u8]) {
        self.texture.update(None, buffer, 64).unwrap();
        self.canvas.copy(&self.texture, None, None).unwrap();
        self.canvas.present()
    }

    fn sleep(&self, duration: u64) {
        std::thread::sleep(Duration::from_millis(duration));
    }

    fn get_key_state(&self) -> [bool; NUM_KEYS] {
        self.key_state.inner()
    }
}

fn keycode_to_keypad(keycode: Option<Keycode>) -> Option<u8> {
    keycode?;

    use Keycode::*;
    let c = match keycode.unwrap() {
        Num1 => '1',
        Num2 => '2',
        Num3 => '3',
        Num4 => '4',
        Q => 'Q',
        W => 'W',
        E => 'E',
        R => 'R',
        A => 'A',
        S => 'S',
        D => 'D',
        F => 'F',
        Z => 'Z',
        X => 'X',
        C => 'C',
        V => 'V',
        _ => '.', // this will turn into an error, end then a None
    };
    keyboard_to_keypad(c).ok()
}
