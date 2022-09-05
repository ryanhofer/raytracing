use crate::vector::{Point3, Vec3};

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}

pub struct HitRecord {
    pub p: Point3,
    pub t: f64,
    pub normal: Vec3,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(t: f64, r: Ray, outward_normal: Vec3) -> Self {
        let p = r.at(t);
        let front_face = r.direction.dot_product(outward_normal) < 0.;

        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            p,
            t,
            normal,
            front_face,
        }
    }
}

pub trait Hit {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
