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