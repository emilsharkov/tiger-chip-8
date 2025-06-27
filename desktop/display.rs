use crate::core::display::Display;
use sdl2::{render::Canvas, video::Window};

pub struct DesktopDisplay {
    canvas: Canvas<Window>,
}

impl Display<Canvas<Window>> for DesktopDisplay {
    fn new(canvas: Canvas<Window>) -> Self {
        Self { canvas }
    }

    fn draw_screen(&mut self, pixels: &[bool]) {
        todo!()
    }
}
