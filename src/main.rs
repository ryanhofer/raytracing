use crate::ray::Ray;
use crate::vector::{Color, Point3, Vec3};
use std::io::{stderr, Write};

pub mod ray;
pub mod vector;

fn main() {
    // Image

    let aspect_ratio = 16. / 9.;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;

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

            let pixel_color = ray_color(r);
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

fn ray_color(r: Ray) -> Color {
    let t = hit_sphere(Point3::new(0., 0., -1.), 0.5, r);
    if t > 0. {
        let n = (r.at(t) - Vec3::new(0., 0., -1.)).unit_vector();
        return Color::new(n.x() + 1., n.y() + 1., n.z() + 1.) * 0.5;
    }

    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.);
    Vec3::new(1., 1., 1.) * (1. - t) + Vec3::new(0.5, 0.7, 1.) * t
}

fn hit_sphere(center: Point3, radius: f64, r: Ray) -> f64 {
    let oc = r.origin - center;

    let a = r.direction.length_squared();
    let half_b = oc.dot_product(r.direction);
    let c = oc.length_squared() - radius * radius;

    let discriminant = half_b * half_b - a * c;
    if discriminant < 0. {
        return -1.;
    }

    return (-half_b - discriminant.sqrt()) / a;
}
