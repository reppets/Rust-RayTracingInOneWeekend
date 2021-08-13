mod vec3;
mod color;
mod ray;
mod hittable;
mod hittable_list;
mod camera;
mod material;

use std::rc::{Rc};

use rand::prelude::*;

use crate::vec3::{Vec3, Color};
use crate::ray::{Ray};
use crate::hittable::{Hittable, Sphere};
use crate::hittable_list::{HittableList};
use crate::camera::{Camera};
use crate::material::{Lambertian, Metal, Dielectric};

fn main() {

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i64;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian{albedo: Vec3(0.8, 0.8, 0.0)});
    let material_center = Rc::new(Lambertian{albedo: Vec3(0.1, 0.2, 0.5)});
    let material_left = Rc::new(Dielectric{ir:1.5});
    let material_right = Rc::new(Metal{albedo: Vec3(0.8, 0.6, 0.2), fuzz:1.0});

    world.add(Box::new(Sphere{center:Vec3(0.0, -100.5, -1.0), radius:100.0, material: material_ground}));
    world.add(Box::new(Sphere{center:Vec3(0.0, 0.0, -1.0), radius:0.5, material: material_center}));
    world.add(Box::new(Sphere{center:Vec3(-1.0, 0.0, -1.0), radius:0.5, material: material_left.clone()}));
    world.add(Box::new(Sphere{center:Vec3(-1.0, 0.0, -1.0), radius:-0.4, material: material_left}));
    world.add(Box::new(Sphere{center:Vec3(1.0, 0.0, -1.0), radius:0.5, material: material_right}));

    // Camera
    let camera = Camera::new();

    // Render
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for y in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", y);
        for x in 0..image_width {
            let mut pixel_color = Vec3(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (x as f64 +  rand::thread_rng().gen_range(0.0..1.0)) / (image_width - 1) as f64;
                let v = (y as f64 + rand::thread_rng().gen_range(0.0..1.0)) / (image_height - 1) as f64;
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }
            color::write_color(&mut std::io::stdout(), &pixel_color, samples_per_pixel);
        }
    }
    eprintln!("\nDone.");
}

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i64) -> Color {

    if depth <= 0 {
        return Vec3(0.0, 0.0, 0.0)
    }

    let hit = world.hit(r, 0.0001, f64::INFINITY);
    hit.map_or_else(
        || {
            let v = 0.5*(r.direction.unit_vector().1 + 1.0);
            (1.0 - v) * Vec3(1.0, 1.0, 1.0) + v * Vec3(0.5, 0.7, 1.0)
        },
        |rec| {rec.material.scatter(r, &rec).map_or_else(
            || {Vec3(0.0, 0.0, 0.0)},
            |(scattered, attenuation)| {
                attenuation * ray_color(&scattered, world, depth-1)
            }
        )})
}