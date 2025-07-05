use tiger_chip_8_core::display::Display;
use web_sys::CanvasRenderingContext2d;

pub struct WasmDisplay {
    canvas: CanvasRenderingContext2d,
}

impl Display<CanvasRenderingContext2d> for WasmDisplay {
    fn new(canvas: CanvasRenderingContext2d) -> Self {
        Self { canvas }
    }

    fn draw_screen(&mut self, pixels: &[bool], width: usize, scale: u8) {
        let canvas_width = self.canvas.canvas().unwrap().width() as f64;
        let canvas_height = self.canvas.canvas().unwrap().height() as f64;
        self.canvas.set_fill_style_str("white");
        self.canvas.clear_rect(0.0, 0.0, canvas_width, canvas_height);

        self.canvas.set_fill_style_str("black");

        for (i, pixel) in pixels.iter().enumerate() {
            if *pixel {
                let x = (i % width) as u32;
                let y = (i / width) as u32;
                let scaled_x = x * scale as u32;
                let scaled_y = y * scale as u32;
                self.canvas.fill_rect(scaled_x as f64, scaled_y as f64, scale as f64, scale as f64)
            }
        }
    }
}
