use crate::vec3::{Point, Vec3};
use crate::ray::{Ray};

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {

    pub fn new(lookfrom: Point, lookat: Point, vup: Vec3, vfov: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64) -> Self {
        let theta = vfov.to_radians();
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom -lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        Self {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: origin - horizontal/2.0 - vertical/2.0 - focus_dist * w,
            u: u,
            v: v,
            w: w,
            lens_radius: aperture / 2.0
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.0 + self.v * rd.1;

        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - offset,
        }
    }
}

