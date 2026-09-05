#[derive(Clone, Copy)]
pub struct Material {
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
}
impl Material {
    pub const fn new(ambient: f32, diffuse: f32, specular: f32, shininess: f32) -> Self {
        Self {
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }
}
