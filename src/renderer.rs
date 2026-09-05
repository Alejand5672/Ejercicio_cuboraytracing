use crate::{
    cube::{Cube, uv_at},
    framebuffer::Framebuffer,
    ray::Ray,
    vec3::Vec3,
};
use raylib::prelude::Color;
pub fn render(fb: &mut Framebuffer, c: &Cube) {
    fb.clear(Color::new(10, 16, 38, 255));
    let (w, h) = (fb.width() as f32, fb.height() as f32);
    let cam = Vec3::new(0.0, 0.0, 0.0);
    let lamp = Vec3::new(-3.5, 4.8, -1.5);
    for y in 0..fb.height() {
        for x in 0..fb.width() {
            let ray = Ray {
                origin: cam,
                direction: Vec3::new(
                    (2.0 * ((x as f32 + 0.5) / w) - 1.0) * (w / h),
                    1.0 - 2.0 * ((y as f32 + 0.5) / h),
                    -1.35,
                )
                .normalize(),
            };
            if let Some(hit) = c.intersect(&ray) {
                let (u, v) = uv_at(hit.local_point, hit.local_normal, c.half_size);
                fb.set_pixel(
                    x,
                    y,
                    shade(
                        c.material.texture.sample(u, v),
                        hit.point,
                        hit.normal,
                        cam,
                        lamp,
                        c,
                    ),
                );
            } else if let Some(p) = floor(&ray) {
                fb.set_pixel(
                    x,
                    y,
                    shade(
                        Color::new(39, 52, 83, 255),
                        p,
                        Vec3::new(0.0, 1.0, 0.0),
                        cam,
                        lamp,
                        c,
                    ),
                );
            }
        }
    }
}
fn floor(ray: &Ray) -> Option<Vec3> {
    let t = (-1.72 - ray.origin.y) / ray.direction.y;
    if t > 0.001 {
        Some(ray.origin + ray.direction * t)
    } else {
        None
    }
}
fn shade(a: Color, p: Vec3, n: Vec3, cam: Vec3, lamp: Vec3, c: &Cube) -> Color {
    let to = lamp - p;
    let dist = to.dot(to).sqrt();
    let l = to / dist;
    let shadow = Ray {
        origin: p + n * 0.004,
        direction: l,
    };
    let blocked = c.intersect(&shadow).is_some_and(|h| h.distance < dist);
    let diffuse = if blocked {
        0.0
    } else {
        n.dot(l).max(0.0) * c.material.diffuse
    };
    let spec = if blocked {
        0.0
    } else {
        (-l).reflect(n)
            .dot((cam - p).normalize())
            .max(0.0)
            .powf(c.material.shininess)
            * c.material.specular
    };
    let b = (c.material.ambient + diffuse).min(1.0);
    Color::new(
        (a.r as f32 * b + 255.0 * spec).min(255.0) as u8,
        (a.g as f32 * b + 255.0 * spec).min(255.0) as u8,
        (a.b as f32 * b + 255.0 * spec).min(255.0) as u8,
        255,
    )
}
