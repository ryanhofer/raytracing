use crate::ray::{Hit, HitRecord, Material, Ray};
use crate::vector::Point3;

pub struct World {
    objects: Vec<Box<dyn Hit + Sync>>,
}

impl World {
    pub fn new(objects: Vec<Box<dyn Hit + Sync>>) -> Self {
        Self { objects }
    }
}

impl Hit for World {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_hit = None;
        let mut t_max = t_max;

        for object in self.objects.iter() {
            if let Some(hit) = object.hit(r, t_min, t_max) {
                t_max = hit.t;
                closest_hit = Some(hit);
            }
        }

        closest_hit
    }
}

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot_product(r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let roots = [
            (-half_b - sqrtd) / a, // 1st root
            (-half_b + sqrtd) / a, // 2nd root
        ];

        // Find the nearest root within the specified range (t_min, t_max)
        let t = match roots.into_iter().find(|&t| t_min <= t && t <= t_max) {
            Some(t) => t,
            None => return None,
        };

        let p = r.at(t);
        let outward_normal = (p - self.center) / self.radius;

        Some(HitRecord::new(t, r, outward_normal, &self.material))
    }
}
