use crate::ray::{Hit, HitRecord, Ray};
use crate::vector::Point3;

pub struct AABB {
    pub min: Point3,
    pub max: Point3,
}

impl AABB {
    pub const fn new(min: Point3, max: Point3) -> Self {
        Self { min, max }
    }

    pub fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> bool {
        // X axis
        let inv_d = r.direction.x().recip();
        let t0 = (self.min.x() - r.origin.x()) * inv_d;
        let t1 = (self.max.x() - r.origin.x()) * inv_d;
        let (t0, t1) = if inv_d < 0. { (t1, t0) } else { (t0, t1) };
        let t_min = if t0 > t_min { t0 } else { t_min };
        let t_max = if t1 < t_max { t1 } else { t_max };
        if t_max <= t_min {
            return false;
        }

        // Y axis
        let inv_d = r.direction.y().recip();
        let t0 = (self.min.y() - r.origin.y()) * inv_d;
        let t1 = (self.max.y() - r.origin.y()) * inv_d;
        let (t0, t1) = if inv_d < 0. { (t1, t0) } else { (t0, t1) };
        let t_min = if t0 > t_min { t0 } else { t_min };
        let t_max = if t1 < t_max { t1 } else { t_max };
        if t_max <= t_min {
            return false;
        }

        // Z axis
        let inv_d = r.direction.z().recip();
        let t0 = (self.min.z() - r.origin.z()) * inv_d;
        let t1 = (self.max.z() - r.origin.z()) * inv_d;
        let (t0, t1) = if inv_d < 0. { (t1, t0) } else { (t0, t1) };
        let t_min = if t0 > t_min { t0 } else { t_min };
        let t_max = if t1 < t_max { t1 } else { t_max };
        if t_max <= t_min {
            return false;
        }

        true
    }
}

impl std::ops::Add for AABB {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let x = self.min.x().min(rhs.min.x());
        let y = self.min.y().min(rhs.min.y());
        let z = self.min.z().min(rhs.min.z());
        let min = Point3::new(x, y, z);

        let x = self.max.x().max(rhs.max.x());
        let y = self.max.y().max(rhs.max.y());
        let z = self.max.z().max(rhs.max.z());
        let max = Point3::new(x, y, z);

        AABB::new(min, max)
    }
}

pub struct BVH {
    left: Box<dyn Hit>,
    right: Box<dyn Hit>,
    bounds: AABB,
}

impl BVH {}

impl Hit for BVH {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bounds.hit(r, t_min, t_max) {
            return None;
        }

        let hit_left = self.left.hit(r, t_min, t_max);

        let t_max = if let Some(hit) = hit_left {
            hit.t
        } else {
            t_max
        };

        let hit_right = self.right.hit(r, t_min, t_max);

        hit_right.or(hit_left)
    }

    fn bounds(&self, time: (f64, f64)) -> Option<AABB> {
        None
    }
}
