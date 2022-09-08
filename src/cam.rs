use rand::Rng;

use crate::ray::Ray;
use crate::vector::{random_in_unit_disk, Point3, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
    time: (f64, f64),
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        view_up: Vec3,
        vertical_fov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_distance: f64,
        time: (f64, f64),
    ) -> Self {
        let theta = vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit_vector();
        let u = view_up.cross_product(w).unit_vector();
        let v = w.cross_product(u);

        let origin = look_from;
        let horizontal = u * viewport_width * focus_distance;
        let vertical = v * viewport_height * focus_distance;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_distance;

        let lens_radius = aperture / 2.0;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
            time,
        }
    }

    pub fn get_ray<T: Rng>(&self, rng: &mut T, s: f64, t: f64) -> Ray {
        let rd = random_in_unit_disk(rng) * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();

        let origin = self.origin + offset;
        let direction =
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset;

        let time = rng.gen_range(self.time.0..self.time.1);

        Ray::new(origin, direction, time)
    }
}
