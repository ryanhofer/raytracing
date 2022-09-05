pub mod cam;
pub mod color;
pub mod ray;
pub mod vector;
pub mod world;

use crate::cam::Camera;
use crate::color::{BLACK, WHITE};
use crate::ray::{Hit, Material, Ray, ScatterResult};
use crate::vector::{Color, Point3, Vec3};
use crate::world::{Sphere, World};

use rand::prelude::*;
use std::f64::consts::PI;
use std::io::{stderr, Write};

fn main() {
    let mut rng = thread_rng();

    // Image

    let aspect_ratio = 16. / 9.;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let radius: f64 = (PI / 4.).cos();

    let material_left = Material::Lambertian {
        albedo: Color::new(0., 0., 1.),
    };
    let material_right = Material::Lambertian {
        albedo: Color::new(1., 0., 0.),
    };

    let world = World::new(vec![
        Box::new(Sphere::new(
            Point3::new(-radius, 0., -1.),
            radius,
            material_left,
        )),
        Box::new(Sphere::new(
            Point3::new(radius, 0., -1.),
            radius,
            material_right,
        )),
    ]);

    // Camera
    let camera = Camera::new(90., aspect_ratio);

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
        return BLACK;
    }

    if let Some(hit) = world.hit(r, 0.001, std::f64::INFINITY) {
        if let Some(ScatterResult {
            scattered,
            attenuation,
        }) = hit.material.scatter(rng, r, hit)
        {
            return attenuation * ray_color(rng, scattered, world, depth - 1);
        }

        return BLACK;
    }

    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.);
    WHITE * (1. - t) + Color::new(0.5, 0.7, 1.) * t
}
