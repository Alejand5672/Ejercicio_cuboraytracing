use raylib::prelude::Color;
pub struct Texture {
    width: u32,
    height: u32,
    pixels: Vec<Color>,
}
impl Texture {
    pub fn from_png(path: &str) -> Result<Self, image::ImageError> {
        let image = image::open(path)?.to_rgba8();
        let (width, height) = image.dimensions();
        let pixels = image
            .pixels()
            .map(|p| Color::new(p[0], p[1], p[2], p[3]))
            .collect();
        Ok(Self {
            width,
            height,
            pixels,
        })
    }
    pub fn sample(&self, u: f32, v: f32) -> Color {
        let x = (u.rem_euclid(1.0) * self.width as f32) as u32 % self.width;
        let y = ((1.0 - v.rem_euclid(1.0)) * self.height as f32) as u32 % self.height;
        self.pixels[(y * self.width + x) as usize]
    }
}
