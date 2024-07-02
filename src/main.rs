#![allow(unused)]

use std::{io::{self}, f64::consts::PI};
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

    // Camera settings
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i32 = 400;
    let vfov: f64 = 90.0;
    let lookfrom: Point3 = Point3::new(-2.0, 2.0, 1.0);
    let lookat: Point3 = Point3::new(0.0, 0.0, -1.0);
    let vup: Point3 = Point3::new(0.0, 1.0, 0.0);

    // Materials
    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.50);
    let material_bubble = Dielectric::new(1.0/1.50);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

    // World
    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, material_center)));
    world.add(Arc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.add(Arc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.4, material_bubble)));
    world.add(Arc::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right)));

    let mut cam: Camera = Camera::new(aspect_ratio, image_width, vfov, lookfrom, lookat, vup);

    cam.render(&world, "output.ppm");
}
