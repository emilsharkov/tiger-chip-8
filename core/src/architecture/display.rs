pub trait Display<Canvas> {
    fn new(canvas: Canvas) -> Self;
    fn draw_screen(&mut self, pixels: &[bool]);
}