pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;

pub struct Vram {
    pub pixels: [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT],
}

impl Vram {
    pub fn new() -> Self {
        Self {
            pixels: [false; DISPLAY_WIDTH * DISPLAY_HEIGHT],
        }
    }

    pub fn clear(&mut self) {
        self.pixels.fill(false);
    }

    pub fn get_pixel(&self, index: usize) -> bool {
        self.pixels[index]
    }

    pub fn update_pixel(&mut self, index: usize, value: bool) {
        self.pixels[index] = value;
    }
}
