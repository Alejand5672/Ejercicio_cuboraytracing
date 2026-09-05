use crate::{material::Material, ray::Ray, vec3::Vec3};
pub struct Hit {
    pub distance: f32,
    pub normal: Vec3,
    pub local_normal: Vec3,
    pub point: Vec3,
    pub local_point: Vec3,
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
        let o = (ray.origin - self.center)
            .rotate_y(-self.rotation.y)
            .rotate_x(-self.rotation.x);
        let d = ray
            .direction
            .rotate_y(-self.rotation.y)
            .rotate_x(-self.rotation.x);
        let (mut near, mut far) = (-f32::INFINITY, f32::INFINITY);
        for (a, b) in [(o.x, d.x), (o.y, d.y), (o.z, d.z)] {
            if b.abs() < 0.00001 {
                if a.abs() > self.half_size {
                    return None;
                }
                continue;
            }
            let t0 = (-self.half_size - a) / b;
            let t1 = (self.half_size - a) / b;
            near = near.max(t0.min(t1));
            far = far.min(t0.max(t1));
        }
        if near > far || far < 0.001 {
            return None;
        }
        let distance = if near > 0.001 { near } else { far };
        let p = o + d * distance;
        let local_n = local_normal(p);
        let n = local_n
            .rotate_x(self.rotation.x)
            .rotate_y(self.rotation.y)
            .normalize();
        Some(Hit {
            distance,
            normal: n,
            local_normal: local_n,
            point: ray.origin + ray.direction * distance,
            local_point: p,
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
pub fn uv_at(local: Vec3, normal: Vec3, half: f32) -> (f32, f32) {
    let s = 0.5 / half;
    if normal.x.abs() > 0.5 {
        (local.z * s + 0.5, local.y * s + 0.5)
    } else if normal.y.abs() > 0.5 {
        (local.x * s + 0.5, local.z * s + 0.5)
    } else {
        (local.x * s + 0.5, local.y * s + 0.5)
    }
}
