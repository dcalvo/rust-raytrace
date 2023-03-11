use crate::ray::{HitRecord, Hittable, Ray};
use crate::Point3;

pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(r.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        // Find nearest root between t_min and t_max
        let sqrt_discriminant = discriminant.sqrt();
        let root = (-half_b - sqrt_discriminant) / a;
        if root < t_min || root > t_max {
            let root = (-half_b + sqrt_discriminant) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let hit_point = r.at(root);
        let normal = (hit_point - self.center) / self.radius;
        let outward_normal = r.direction.dot(normal) < 0.0;

        Some(HitRecord {
            t: root,
            p: hit_point,
            normal: if outward_normal { normal } else { -normal },
            outward_normal,
        })
    }
}
