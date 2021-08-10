use crate::ray::{Ray};
use crate::vec3::{Vec3,Point};

pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord {
    fn new(p: Point, t: f64, r: &Ray, outward_normal: Vec3) -> Self {
        let front_face = r.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {outward_normal} else {-outward_normal};
        Self{
            p: p,
            normal: normal,
            t: t,
            front_face: front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(r.direction);
        let c = oc.length_squared() - self.radius*self.radius;

        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return None
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None
            }
        }

        let p = r.at(root);
        let outward_normal = (p - self.center) / self.radius;
        Some(HitRecord::new(p, root, r, outward_normal))
    }
}