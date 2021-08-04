mod vec3;
mod color;
mod ray;
mod hittable;
mod hittable_list;

use crate::vec3::{Vec3, Color};
use crate::ray::{Ray};
use crate::hittable::{Hittable, Sphere};
use crate::hittable_list::{HittableList};

fn main() {

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i64;

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere{center:Vec3(0.0, 0.0, -1.0), radius:0.5}));
    world.add(Box::new(Sphere{center:Vec3(0.0, -100.5, -1.0), radius:100.0}));

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3(0.0, 0.0, 0.0);
    let horizontal = Vec3(viewport_width, 0.0, 0.0);
    let vertical =  Vec3(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3(0.0, 0.0, focal_length);

    // Render
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for y in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", y);
        for x in 0..image_width {
            let u = x as f64 / (image_width - 1) as f64;
            let v = y as f64 / (image_height - 1) as f64;
            let r = Ray{origin: origin, direction: lower_left_corner + u*horizontal + v*vertical - origin};
            let pixel_color = ray_color(&r, &world);
            color::write_color(&mut std::io::stdout(), &pixel_color)
        }
    }
    eprintln!("\nDone.");
}

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let hit = world.hit(r, 0.0, f64::INFINITY);
    hit.map_or_else(
        || {
            let v = 0.5*(r.direction.unit_vector().1 + 1.0);
            (1.0 - v) * Vec3(1.0, 1.0, 1.0) + v * Vec3(0.5, 0.7, 1.0)
        },
        |h| {
            0.5*(h.normal + Vec3(1.0, 1.0, 1.0))
        })
}