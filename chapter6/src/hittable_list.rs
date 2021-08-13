#![allow(unused)]
use crate::ray::{Ray};
use crate::hittable::{HitRecord, Hittable};

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {

    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) -> () {
        self.objects.clear();
    }

    pub fn add(&mut self, o: Box<dyn Hittable>) -> () {
        self.objects.push(o);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r:&Ray, t_min:f64, t_max:f64) -> Option<HitRecord> {
        let mut hit_found: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        for o in self.objects.iter() {
            match o.hit(r, t_min, closest_so_far).map_or((hit_found, closest_so_far), |h| {let t = h.t; (Some(h), t)}) {
                (hit, new_t) => {
                    hit_found = hit;
                    closest_so_far = new_t;
                }
            }
        }
        hit_found
    }
}