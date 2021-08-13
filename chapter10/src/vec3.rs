use std::ops;
use std::ops::{Range};
use std::fmt;
use std::cmp;
use rand::prelude::*;

#[derive(Copy, Clone, PartialEq)]
pub struct Vec3(pub f64, pub f64, pub f64);

fn get_rand() -> f64 {
    rand::thread_rng().gen_range(0.0..1.0)
}

fn get_rand_range(range: Range<f64>) -> f64 {
    rand::thread_rng().gen_range(range)
}

impl Vec3 {

    fn random() -> Self {
        Vec3(get_rand(), get_rand(), get_rand())
    }

    fn random_range(range: Range<f64>) -> Self {
        Vec3(get_rand_range(range.clone()), get_rand_range(range.clone()), get_rand_range(range))
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Vec3::random_range(-1.0..1.0);
            if p.length_squared() >= 1.0 {continue}
            return p;
        }
    }

    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_vector()
    }

    pub fn random_in_hemisphere(normal: Vec3) -> Self {
        let s = Self::random_in_unit_sphere();
        if s.dot(normal) > 0.0 {s} else {-s}
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.0*self.0 + self.1*self.1 + self.2*self.2
    }

    pub fn dot(self, rhs: Vec3) -> f64 {
        self.0*rhs.0 + self.1*rhs.1 + self.2*rhs.2
    }

    pub fn cross(self, rhs: Vec3) -> Vec3 {
        Vec3(
            self.1*rhs.2 - self.2 * rhs.1,
            self.2*rhs.0 - self.0 * rhs.2,
            self.0*rhs.1 - self.1 * rhs.0
        )
    }

    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
    }

    pub fn is_near_zero(&self) -> bool {
        let s = 1e-8;
        self.0.abs() < s && self.1.abs() < s && self.2.abs() < s
    }

    pub fn reflect(&self, n: Vec3) -> Vec3 {
        let v = self.unit_vector();
        v - 2.0*v.dot(n)*n
    }

    pub fn refract(&self, n: Vec3, etai_over_etat:f64) -> Vec3 {
        let cos_theta = cmp::min_by((-self).dot(n), 1.0, |x,y| {x.partial_cmp(y).unwrap()});
        let r_out_perp = etai_over_etat * (self + cos_theta*n);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
        r_out_perp + r_out_parallel
    }

}

/*
    Operators for Vec3
*/
impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl ops::Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        (1.0/rhs) * self
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        * self = Self(self.0+rhs.0, self.1+rhs.1, self.2+rhs.2)
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        * self = Self(self.0*rhs, self.1*rhs, self.2*rhs)
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        * self *= 1.0/rhs;
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}

pub type Point = Vec3;
pub type Color = Vec3;

