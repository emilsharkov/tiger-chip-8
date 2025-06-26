use crate::core::display::Display;
use sdl2::{render::Canvas, video::Window};

pub struct DesktopDisplay {
    canvas: Canvas<Window>,
}

impl Display for DesktopDisplay {
    type Canvas = Canvas<Window>;
    fn new(canvas: Self::Canvas) -> Self {
        Self { canvas }
    }

    fn draw_screen(&mut self, pixels: &[bool]) {
    }
}
