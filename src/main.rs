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

    let aspect_ratio = 3. / 2.;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 500;
    let max_depth = 50;

    // World

    let world = random_scene(&mut rng);

    // Camera

    let look_from = Point3::new(13., 2., 3.);
    let look_at = Point3::zero();
    let view_up = Vec3::new(0., 1., 0.);
    let vertical_fov = 20.;
    let distance_to_focus = 10.;
    let aperture = 0.1;

    let camera = Camera::new(
        look_from,
        look_at,
        view_up,
        vertical_fov,
        aspect_ratio,
        aperture,
        distance_to_focus,
    );

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
                let r = camera.get_ray(&mut rng, u, v);

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

fn random_scene<T: Rng>(rng: &mut T) -> World {
    let mut objects: Vec<Box<dyn Hit>> = vec![];

    let ground_material = Material::Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    };

    objects.push(Box::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let a = a as f64;
            let b = b as f64;

            let choose_mat = rng.gen::<f64>();

            let x = a + 0.9 * rng.gen::<f64>();
            let y = 0.2;
            let z = b + 0.9 * rng.gen::<f64>();

            let center = Point3::new(x, y, z);
            let p = Point3::new(4., 0.2, 0.);

            if (center - p).length() > 0.9 {
                let sphere_material = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random(rng) * Color::random(rng);
                    Material::Lambertian { albedo }
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(rng, 0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    Material::Metal { albedo, fuzz }
                } else {
                    // glass
                    Material::Dialectric {
                        index_of_refraction: 1.5,
                    }
                };

                objects.push(Box::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    let dialectric = Material::Dialectric {
        index_of_refraction: 1.5,
    };
    objects.push(Box::new(Sphere::new(
        Point3::new(0., 1., 0.),
        1.,
        dialectric,
    )));

    let lambertian = Material::Lambertian {
        albedo: Color::new(0.4, 0.2, 0.1),
    };
    objects.push(Box::new(Sphere::new(
        Point3::new(-4., 1., 0.),
        1.,
        lambertian,
    )));

    let metal = Material::Metal {
        albedo: Color::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    };
    objects.push(Box::new(Sphere::new(Point3::new(4., 1., 0.), 1., metal)));

    World::new(objects)
}
