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
use crate::material::{Lambertian, Metal, Dielectric, Material};

fn main() {

    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as i64;
    let samples_per_pixel = 500;
    let max_depth = 50;

    // World
    let mut world = HittableList::new();

    let ground_material = Rc::new(Lambertian{albedo: Vec3(0.5, 0.5, 0.5)});
    world.add(Box::new(Sphere{center:Vec3(0.0, -1000.0, 0.0), radius:1000.0, material: ground_material}));

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen_range(0.0..1.0);
            let center = Vec3(a as f64 + 0.9+rng.gen_range(0.0..1.0), 0.2, b as f64 + 0.9*rng.gen_range(0.0..1.0));

            if (center - Vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                let mat:Rc<dyn Material> = if choose_mat < 0.8 {
                    Rc::new(Lambertian{albedo: Vec3::random() * Vec3::random()})
                } else if choose_mat < 0.95 {
                    Rc::new(Metal{albedo: Vec3::random_range(0.0..0.5), fuzz: rng.gen_range(0.0..0.5)})
                } else {
                    Rc::new(Dielectric{ir: 1.5})
                };
                world.add(Box::new(Sphere{center:center, radius:0.2, material: mat}));
            }
        }
    }
    
    let material1 = Rc::new(Dielectric{ir:1.5});
    world.add(Box::new(Sphere{center:Vec3(0.0, 1.0, 0.0), radius:1.0, material: material1}));

    let material2 = Rc::new(Lambertian{albedo: Vec3(0.4, 0.2, 0.1)});
    world.add(Box::new(Sphere{center:Vec3(-4.0, 1.0, 0.0), radius:1.0, material: material2}));

    let material3 = Rc::new(Metal{albedo: Vec3(0.7, 0.6, 0.5), fuzz: 0.0});
    world.add(Box::new(Sphere{center:Vec3(4.0, 1.0, 0.0), radius:1.0, material: material3}));

    // Camera
    let lookfrom = Vec3(13.0, 2.0, 3.0);
    let lookat = Vec3(0.0, 0.0, 0.0);
    let camera = Camera::new(lookfrom, lookat, Vec3(0.0, 1.0, 0.0), 20.0, aspect_ratio, 0.1, 10.0);

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