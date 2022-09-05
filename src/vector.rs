use std::ops::Neg;

use rand::Rng;

#[derive(Clone, Copy)]
pub struct Vec3(pub f64, pub f64, pub f64);

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

    pub const fn zero() -> Self {
        Self(0., 0., 0.)
    }

    pub fn random<T: Rng>(rng: &mut T) -> Self {
        Self(rng.gen(), rng.gen(), rng.gen())
    }

    pub fn random_range<T: Rng>(rng: &mut T, min: f64, max: f64) -> Self {
        let x = rng.gen_range(min..max);
        let y = rng.gen_range(min..max);
        let z = rng.gen_range(min..max);
        Self(x, y, z)
    }

    pub fn x(self) -> f64 {
        self.0
    }

    pub fn y(self) -> f64 {
        self.1
    }

    pub fn z(self) -> f64 {
        self.2
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn dot_product(self, other: Self) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross_product(self, other: Self) -> Self {
        Self(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn unit_vector(self) -> Self {
        self / self.length()
    }

    pub fn reflect(self, normal: Vec3) -> Self {
        self - normal * self.dot_product(normal) * 2.
    }

    pub fn refract(self, normal: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = (-self).dot_product(normal).min(1.);
        let r_perp = (self + normal * cos_theta) * etai_over_etat;
        let r_parallel = normal * (1. - r_perp.length_squared()).abs().sqrt().neg();
        r_perp + r_parallel
    }

    pub fn near_zero(self, epsilon: f64) -> bool {
        self.0.abs() < epsilon && self.1.abs() < epsilon && self.2.abs() < epsilon
    }
}

// Unary Operators

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

// Binary Operators

impl std::ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Self) -> Self::Output {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl std::ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Self::Output {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl std::ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        *self = Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl std::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self(self.0 * rhs, self.1 * rhs, self.2 * rhs);
    }
}

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl std::ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        *self = Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2);
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl std::ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = Self(self.0 / rhs, self.1 / rhs, self.2 / rhs);
    }
}

pub fn random_in_unit_sphere<T: Rng>(rng: &mut T) -> Vec3 {
    loop {
        let p = Vec3::random_range(rng, -1., 1.);
        if p.length_squared() >= 1. {
            continue;
        }
        return p;
    }
}

pub fn random_unit_vector<T: Rng>(rng: &mut T) -> Vec3 {
    random_in_unit_sphere(rng).unit_vector()
}
