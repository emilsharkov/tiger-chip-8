const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

pub struct Display {
    pub pixels: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
}

impl Display {
    pub fn new() -> Self {
        Self { pixels: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT] }
    }

    pub fn clear(&mut self) {
        self.pixels.fill([false; DISPLAY_WIDTH]);
    }

    pub fn update_pixel(&mut self, x: usize, y: usize, value: bool) {
        let row_index = y % DISPLAY_HEIGHT;
        let col_index = x % DISPLAY_WIDTH;
        self.pixels[row_index][col_index] = value;
    }
}