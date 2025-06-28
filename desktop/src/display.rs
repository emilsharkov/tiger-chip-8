use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};
use tiger_chip8_core::display::Display;

pub struct DesktopDisplay {
    canvas: Canvas<Window>,
}

impl Display<Canvas<Window>> for DesktopDisplay {
    fn new(canvas: Canvas<Window>) -> Self {
        Self { canvas }
    }

    fn draw_screen(&mut self, pixels: &[bool], width: usize, scale: u8) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        for (i, pixel) in pixels.iter().enumerate() {
            if *pixel {
                let x = (i % width) as u32;
                let y = (i / width) as u32;
                let scaled_x = x * scale as u32;
                let scaled_y = y * scale as u32;
                let scaled_pixel =
                    Rect::new(scaled_x as i32, scaled_y as i32, scale as u32, scale as u32);
                self.canvas.fill_rect(scaled_pixel).unwrap();
            }
        }
        self.canvas.present();
    }
}
