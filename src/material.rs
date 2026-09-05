use crate::texture::Texture;
use raylib::prelude::Color;

/// Todo lo que consulta el sombreado al golpear una superficie.
pub struct Material {
    pub albedo: Color,
    pub texture: Texture,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
}
impl Material {
    pub const fn textured(
        texture: Texture,
        ambient: f32,
        diffuse: f32,
        specular: f32,
        shininess: f32,
    ) -> Self {
        Self {
            albedo: Color::WHITE,
            texture,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }
}
