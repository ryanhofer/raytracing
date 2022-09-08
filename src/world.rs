use crate::bounds::AABB;
use crate::ray::{Hit, HitRecord, Material, Ray};
use crate::vector::{Point3, Vec3};

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

    fn bounds(&self, time: (f64, f64)) -> Option<AABB> {
        self.objects
            .iter()
            .filter_map(|obj| obj.bounds(time))
            .reduce(|sum, item| sum + item)
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

    fn bounds(&self, time: (f64, f64)) -> Option<crate::bounds::AABB> {
        let octant = Vec3::new(self.radius, self.radius, self.radius);

        Some(AABB::new(self.center - octant, self.center + octant))
    }
}

pub struct MovingSphere {
    pub time: (f64, f64),
    pub center: (Point3, Point3),
    pub radius: f64,
    pub material: Material,
}

impl MovingSphere {
    pub fn new(
        time: (f64, f64),
        center: (Point3, Point3),
        radius: f64,
        material: Material,
    ) -> Self {
        Self {
            time,
            center,
            radius,
            material,
        }
    }

    pub fn center(&self, time: f64) -> Point3 {
        let t_delta = time - self.time.0;
        let t_range = self.time.1 - self.time.0;
        let c_delta = self.center.1 - self.center.0;
        self.center.0 + c_delta * (t_delta / t_range)
    }
}

impl Hit for MovingSphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center(r.time);
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
        let outward_normal = (p - self.center(r.time)) / self.radius;

        Some(HitRecord::new(t, r, outward_normal, &self.material))
    }

    fn bounds(&self, time: (f64, f64)) -> Option<AABB> {
        let c0 = self.center(time.0);
        let c1 = self.center(time.1);
        let octant = Vec3::new(self.radius, self.radius, self.radius);

        let a = AABB::new(c0 - octant, c0 + octant);
        let b = AABB::new(c1 - octant, c1 + octant);

        Some(a + b)
    }
}
