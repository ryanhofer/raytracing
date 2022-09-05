use crate::ray::{Hit, Ray, Sphere};
use crate::vector::{Color, Point3, Vec3};
use std::io::{stderr, Write};

pub mod ray;
pub mod vector;

fn main() {
    // Image

    let aspect_ratio = 16. / 9.;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;

    // World
    let world = World {
        objects: vec![
            Box::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)),
            Box::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)),
        ],
    };

    // Camera

    let viewport_height = 2.;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.;

    let origin: Point3 = Vec3::zero();
    let horizontal = Vec3::new(viewport_width, 0., 0.);
    let vertical = Vec3::new(0., viewport_height, 0.);
    let lower_left_corner =
        origin - horizontal / 2. - vertical / 2. - Vec3::new(0., 0., focal_length);

    // Render

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {} ", j);
        stderr().flush().expect("failed to flush stderr");

        for i in 0..image_width {
            let j = j as f64;
            let i = i as f64;

            let u = i / (image_width - 1) as f64;
            let v = j / (image_height - 1) as f64;
            let direction = lower_left_corner + horizontal * u + vertical * v - origin;
            let r = Ray::new(origin, direction);

            let pixel_color = ray_color(r, &world);
            write_color(pixel_color);
        }
    }

    eprintln!("Done.");
}

fn write_color(pixel_color: Color) {
    let r = (255.999 * pixel_color.0) as i32;
    let g = (255.999 * pixel_color.1) as i32;
    let b = (255.999 * pixel_color.2) as i32;
    println!("{} {} {}", r, g, b);
}

fn ray_color(r: Ray, world: &World) -> Color {
    if let Some(hit) = world.hit(r, 0., std::f64::INFINITY) {
        return (Color::new(1., 1., 1.) + hit.normal) * 0.5;
    }

    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.);
    Color::new(1., 1., 1.) * (1. - t) + Color::new(0.5, 0.7, 1.) * t
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
