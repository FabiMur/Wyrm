#![allow(unused)]

use std::io::{self};
use env_logger;

mod camera;
mod hittable;
mod utils;
mod primitives;
mod materials;

use std::sync::Arc;
use primitives::*;
use materials::{Lambertian, Metal, Dielectric};
use hittable::hittable_list::{HittableList};
use primitives::sphere::Sphere;
use camera::Camera;

fn main() {
    env_logger::init();


    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i32 = 720;
    // Materials
    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0)); // Create a red Lambertian material
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5)); // Create a red Lambertian material
    let material_left = Dielectric::new(1.5); // Create a red Lambertian material
    let material_bubble = Dielectric::new(1.0/1.5); // Create a red Lambertian material
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0); // Create a red Lambertian material

    // World
    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground.clone())));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, material_center.clone())));
    world.add(Arc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left.clone())));
    world.add(Arc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.4, material_bubble.clone())));
    world.add(Arc::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right.clone())));

    let cam: Camera = Camera::new(aspect_ratio, image_width);
    let _result = cam.render(&world, "output.ppm");
}
