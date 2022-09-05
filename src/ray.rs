use rand::Rng;

use crate::color;
use crate::vector::{random_in_unit_sphere, random_unit_vector, Color, Point3, Vec3};

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

pub struct HitRecord<'a> {
    pub p: Point3,
    pub t: f64,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: &'a Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(t: f64, r: Ray, outward_normal: Vec3, material: &'a Material) -> Self {
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
            material,
        }
    }
}

pub trait Hit {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub enum Material {
    Dialectric { index_of_refraction: f64 },
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f64 },
}

impl Material {
    pub fn scatter<T: Rng>(&self, rng: &mut T, r: Ray, hit: HitRecord) -> Option<ScatterResult> {
        match self {
            &Material::Dialectric {
                index_of_refraction,
            } => {
                let refraction_ratio = if hit.front_face {
                    index_of_refraction.recip()
                } else {
                    index_of_refraction
                };

                let unit_direction = r.direction.unit_vector();
                let refracted = unit_direction.refract(hit.normal, refraction_ratio);
                let scattered = Ray::new(hit.p, refracted);
                let attenuation = color::WHITE;

                Some(ScatterResult {
                    scattered,
                    attenuation,
                })
            }
            &Material::Lambertian { albedo } => {
                let scatter_direction = hit.normal + random_unit_vector(rng);

                // Catch degenerate scatter direction
                let scatter_direction = if scatter_direction.near_zero(1e-8) {
                    hit.normal
                } else {
                    scatter_direction
                };

                let scattered = Ray::new(hit.p, scatter_direction);
                let attenuation = albedo;

                Some(ScatterResult {
                    scattered,
                    attenuation,
                })
            }

            &Material::Metal { albedo, fuzz } => {
                let reflected = r.direction.unit_vector().reflect(hit.normal);
                let fuzz_offset = random_in_unit_sphere(rng) * fuzz;
                let scattered = Ray::new(hit.p, reflected + fuzz_offset);
                let attenuation = albedo;

                if scattered.direction.dot_product(hit.normal) <= 0. {
                    return None;
                }

                Some(ScatterResult {
                    scattered,
                    attenuation,
                })
            }
        }
    }
}

pub struct ScatterResult {
    pub scattered: Ray,
    pub attenuation: Color,
}
