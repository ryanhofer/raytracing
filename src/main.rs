use crate::ray::{Hit, Ray, Sphere};
use crate::vector::{Color, Point3, Vec3};
use rand::prelude::*;
use std::io::{stderr, Write};

pub mod ray;
pub mod vector;

fn main() {
    let mut rng = thread_rng();

    // Image

    let aspect_ratio = 16. / 9.;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let world = World {
        objects: vec![
            Box::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)),
            Box::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)),
        ],
    };

    // Camera
    let camera = Camera::new();

    // Render

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {} ", j);
        stderr().flush().expect("failed to flush stderr");

        for i in 0..image_width {
            let j = j as f64;
            let i = i as f64;

            let mut pixel_color = Vec3::zero();
            for _ in 0..samples_per_pixel {
                let u = (i + rng.gen::<f64>()) / (image_width - 1) as f64;
                let v = (j + rng.gen::<f64>()) / (image_height - 1) as f64;
                let r = camera.get_ray(u, v);

                pixel_color += ray_color(&mut rng, r, &world, max_depth);
            }

            write_color(pixel_color, samples_per_pixel);
        }

        println!();
    }

    eprintln!("Done.");
}

fn write_color(pixel_color: Color, samples_per_pixel: i32) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    // Divide the color by the number of samples and gamma-correct for gamma=2.0
    let samples_per_pixel = samples_per_pixel as f64;
    let scale = 1. / samples_per_pixel;
    let r = (scale * r).sqrt();
    let g = (scale * g).sqrt();
    let b = (scale * b).sqrt();

    let ir = (256. * r.clamp(0., 0.999)) as i32;
    let ig = (256. * g.clamp(0., 0.999)) as i32;
    let ib = (256. * b.clamp(0., 0.999)) as i32;

    print!("{} {} {} ", ir, ig, ib);
}

fn ray_color<T: Rng>(rng: &mut T, r: Ray, world: &World, depth: i32) -> Color {
    if depth <= 0 {
        return Color::zero();
    }

    if let Some(hit) = world.hit(r, 0.001, std::f64::INFINITY) {
        let target = hit.p + hit.normal + random_unit_vector(rng);
        let scattered_ray = Ray::new(hit.p, target - hit.p);
        return ray_color(rng, scattered_ray, world, depth - 1) * 0.5;
    }

    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.);
    Color::new(1., 1., 1.) * (1. - t) + Color::new(0.5, 0.7, 1.) * t
}

fn random_in_unit_sphere<T: Rng>(rng: &mut T) -> Vec3 {
    loop {
        let p = Vec3::random_range(rng, -1., 1.);
        if p.length_squared() >= 1. {
            continue;
        }
        return p;
    }
}

fn random_unit_vector<T: Rng>(rng: &mut T) -> Vec3 {
    random_in_unit_sphere(rng).unit_vector()
}

struct World {
    objects: Vec<Box<dyn Hit>>,
}

impl Hit for World {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<ray::HitRecord> {
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

struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    fn new() -> Self {
        let aspect_ratio = 16. / 9.;
        let viewport_height = 2.;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.;

        let origin = Point3::zero();
        let horizontal = Vec3::new(viewport_width, 0., 0.);
        let vertical = Vec3::new(0., viewport_height, 0.);
        let lower_left_corner =
            origin - horizontal / 2. - vertical / 2. - Vec3::new(0., 0., focal_length);

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
