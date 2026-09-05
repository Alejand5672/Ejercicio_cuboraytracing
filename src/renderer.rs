use crate::{cube::Cube, framebuffer::Framebuffer, ray::Ray, vec3::Vec3};
use raylib::prelude::Color;
pub fn render(fb: &mut Framebuffer, cube: &Cube) {
    fb.clear(Color::new(10, 16, 38, 255));
    let (w, h) = (fb.width() as f32, fb.height() as f32);
    let camera = Vec3::new(0.0, 0.0, 0.0);
    let light = Vec3::new(-0.75, 0.9, -0.55).normalize();
    for y in 0..fb.height() {
        for x in 0..fb.width() {
            let sx = (2.0 * ((x as f32 + 0.5) / w) - 1.0) * (w / h);
            let sy = 1.0 - 2.0 * ((y as f32 + 0.5) / h);
            let ray = Ray {
                origin: camera,
                direction: Vec3::new(sx, sy, -1.35).normalize(),
            };
            if let Some(hit) = cube.intersect(&ray) {
                let diffuse = hit.normal.dot(light).max(0.0) * cube.material.diffuse;
                let view = (camera - (ray.origin + ray.direction * hit.distance)).normalize();
                let shine = (-light)
                    .reflect(hit.normal)
                    .dot(view)
                    .max(0.0)
                    .powf(cube.material.shininess)
                    * cube.material.specular;
                let b = (cube.material.ambient + diffuse).min(1.0);
                fb.set_pixel(
                    x,
                    y,
                    Color::new(
                        (hit.color.r as f32 * b + 255.0 * shine).min(255.0) as u8,
                        (hit.color.g as f32 * b + 255.0 * shine).min(255.0) as u8,
                        (hit.color.b as f32 * b + 255.0 * shine).min(255.0) as u8,
                        255,
                    ),
                );
            }
        }
    }
}
