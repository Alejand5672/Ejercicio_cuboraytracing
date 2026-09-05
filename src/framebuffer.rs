use raylib::prelude::*;
pub struct Framebuffer {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
    background: Color,
}
impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::BLACK; width * height],
            background: Color::BLACK,
        }
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn clear(&mut self, color: Color) {
        self.background = color;
        self.pixels.fill(color)
    }
    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.pixels[y * self.width + x] = color
    }
    pub fn draw(&self, window: &mut RaylibHandle, thread: &RaylibThread) {
        let sw = window.get_screen_width().max(1) as usize;
        let sh = window.get_screen_height().max(1) as usize;
        let scale = (sw / self.width)
            .min(sh.saturating_sub(48) / self.height)
            .max(1);
        let (dw, dh) = (self.width * scale, self.height * scale);
        let (ox, oy) = ((sw - dw) / 2, 42 + (sh.saturating_sub(48) - dh) / 2);
        let mut d = window.begin_drawing(thread);
        d.clear_background(self.background);
        for y in 0..self.height {
            for x in 0..self.width {
                d.draw_rectangle(
                    (ox + x * scale) as i32,
                    (oy + y * scale) as i32,
                    scale as i32,
                    scale as i32,
                    self.pixels[y * self.width + x],
                );
            }
        }
        d.draw_text(
            "Cubo con textura de cuarzo | Flechas: girar",
            14,
            12,
            20,
            Color::RAYWHITE,
        );
    }
}
