use crate::{material::Material, ray::Ray, vec3::Vec3};
use raylib::prelude::Color;
pub struct Hit {
    pub distance: f32,
    pub normal: Vec3,
    pub color: Color,
}
pub struct Cube {
    pub center: Vec3,
    pub half_size: f32,
    pub rotation: Vec3,
    pub material: Material,
}
impl Cube {
    pub const fn new(center: Vec3, half_size: f32, rotation: Vec3, material: Material) -> Self {
        Self {
            center,
            half_size,
            rotation,
            material,
        }
    }
    pub fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let origin = (ray.origin - self.center)
            .rotate_y(-self.rotation.y)
            .rotate_x(-self.rotation.x);
        let direction = ray
            .direction
            .rotate_y(-self.rotation.y)
            .rotate_x(-self.rotation.x);
        let (mut near, mut far) = (-f32::INFINITY, f32::INFINITY);
        for (o, d) in [
            (origin.x, direction.x),
            (origin.y, direction.y),
            (origin.z, direction.z),
        ] {
            if d.abs() < 0.00001 {
                if o.abs() > self.half_size {
                    return None;
                }
                continue;
            }
            let a = (-self.half_size - o) / d;
            let b = (self.half_size - o) / d;
            near = near.max(a.min(b));
            far = far.min(a.max(b));
        }
        if near > far || far < 0.001 {
            return None;
        }
        let distance = if near > 0.001 { near } else { far };
        let p = origin + direction * distance;
        let n = local_normal(p)
            .rotate_x(self.rotation.x)
            .rotate_y(self.rotation.y)
            .normalize();
        Some(Hit {
            distance,
            normal: n,
            color: face_color(n),
        })
    }
}
fn local_normal(p: Vec3) -> Vec3 {
    let (x, y, z) = (p.x.abs(), p.y.abs(), p.z.abs());
    if x >= y && x >= z {
        Vec3::new(p.x.signum(), 0.0, 0.0)
    } else if y >= z {
        Vec3::new(0.0, p.y.signum(), 0.0)
    } else {
        Vec3::new(0.0, 0.0, p.z.signum())
    }
}
// Colores sólidos por cara; no hay UVs, archivos de imagen ni texturas.
fn face_color(n: Vec3) -> Color {
    if n.y > 0.45 {
        Color::new(255, 186, 73, 255)
    } else if n.x > 0.35 {
        Color::new(255, 86, 135, 255)
    } else if n.x < -0.35 {
        Color::new(78, 203, 255, 255)
    } else if n.z > 0.0 {
        Color::new(117, 101, 255, 255)
    } else {
        Color::new(60, 219, 176, 255)
    }
}
