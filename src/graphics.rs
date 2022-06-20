use sdl2::pixels::Color;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use sdl2::Sdl;

pub struct Graphics {
    canvas: Canvas<Window>,
    texture: Texture,
}

impl Graphics {
    pub fn new(sdl_context: &Sdl, width: u32, height: u32) -> Self {
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("wasm-chip8", width, height)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        let mut canvas = window
            .into_canvas()
            .accelerated()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.present();

        let texture = canvas
            .texture_creator()
            .create_texture_target(sdl2::pixels::PixelFormatEnum::RGB332, 64, 32)
            .unwrap();

        Graphics { canvas, texture }
    }

    pub fn draw(&mut self, buffer: &[u8]) {
        self.texture.update(None, buffer, 64).unwrap();
        self.canvas.copy(&self.texture, None, None).unwrap();
        self.canvas.present();
    }
}
