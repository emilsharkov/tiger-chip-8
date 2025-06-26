pub trait Display {
    type Canvas;
    fn new(canvas: Self::Canvas) -> Self;
    fn draw_screen(&mut self, pixels: &[bool]);
}