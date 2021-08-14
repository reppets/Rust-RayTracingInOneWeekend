use std::cmp;
use rand::prelude::*;

use crate::ray::{Ray};
use crate::vec3::{Vec3, Color};
use crate::hittable::{HitRecord};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    pub albedo: Color
}

impl Material for Lambertian {
    
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.is_near_zero() {
            scatter_direction = rec.normal;
        }
        Some( (Ray{origin:rec.p, direction:scatter_direction}, self.albedo) )
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {

    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = r_in.direction.unit_vector().reflect(rec.normal);
        let scattered = Ray{origin: rec.p, direction: reflected + self.fuzz*Vec3::random_in_unit_sphere()};
        if scattered.direction.dot(rec.normal) > 0.0 {
            Some( (scattered, self.albedo) )
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub ir: f64,
}

impl Dielectric {
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0-ref_idx) / (1.0+ref_idx);
        r0 = r0 * r0;
        r0 + (1.0-r0)*(1.0-cosine).powi(5)
    }
}

impl Material for Dielectric {

    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let refraction_ratio = if rec.front_face {1.0/self.ir} else {self.ir};
        let unit_direction = r_in.direction.unit_vector();
        let cos_thetha = cmp::min_by((-unit_direction).dot(rec.normal), 1.0, |x, y| {x.partial_cmp(y).unwrap()});
        let sin_thetha = (1.0 - cos_thetha * cos_thetha).sqrt();
        let cannot_refract = refraction_ratio * sin_thetha > 1.0;

        let direction = if cannot_refract || (Dielectric::reflectance(cos_thetha, refraction_ratio) > rand::thread_rng().gen_range(0.0..1.0)) {
            unit_direction.reflect(rec.normal)
        } else {
            unit_direction.refract(rec.normal, refraction_ratio)
        };

        Some( (Ray{origin:rec.p, direction:direction}, Vec3(1.0, 1.0, 1.0)) )
    }
}