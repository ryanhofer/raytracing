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
    let material_ground = Material::Lambertian {
        albedo: Color::new(0.8, 0.8, 0.),
    };
    let material_center = Material::Lambertian {
        albedo: Color::new(0.1, 0.2, 0.5),
    };
    let material_left = Material::Dialectric {
        index_of_refraction: 1.5,
    };
    let material_right = Material::Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
        fuzz: 0.0,
    };

    let world = World::new(vec![
        Box::new(Sphere::new(
            Point3::new(0., -100.5, -1.),
            100.,
            material_ground,
        )),
        Box::new(Sphere::new(Point3::new(0., 0., -1.), 0.5, material_center)),
        Box::new(Sphere::new(Point3::new(-1., 0., -1.), 0.5, material_left)),
        Box::new(Sphere::new(Point3::new(-1., 0., -1.), -0.4, material_left)),
        Box::new(Sphere::new(Point3::new(1., 0., -1.), 0.5, material_right)),
    ]);

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
