use std::io::{stderr, Write};

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
    let horizontal = Vec3(viewport_width, 0., 0.);
    let vertical = Vec3(0., viewport_height, 0.);
    let lower_left_corner = origin - horizontal / 2. - vertical / 2. - Vec3(0., 0., focal_length);

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
            let r = Ray { origin, direction };

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
    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (1. + unit_direction.y());
    Vec3(1., 1., 1.) * (1. - t) + Vec3(0.5, 0.7, 1.) * t
}

#[derive(Clone, Copy)]
struct Vec3(f64, f64, f64);
type Point3 = Vec3;
type Color = Vec3;

impl Vec3 {
    fn x(self) -> f64 {
        self.0
    }

    fn y(self) -> f64 {
        self.1
    }

    fn z(self) -> f64 {
        self.2
    }

    fn zero() -> Self {
        Vec3(0., 0., 0.)
    }

    fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    fn length_squared(self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    fn dot_product(self, other: Self) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    fn cross_product(self, other: Self) -> Self {
        Self(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    fn unit_vector(self) -> Self {
        self / self.length()
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl std::ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl std::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1. / rhs)
    }
}

impl std::ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}
