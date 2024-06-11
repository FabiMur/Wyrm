#![allow(unused)]

mod vec3;
mod color;
mod ray;
mod utils;
mod hittable;
mod hittable_list;
mod sphere;
mod camera;
mod interval;

use in_one_weekend::utils::INFINITY;
use log::info;
use std::cmp::max;
use std::io::{self};
use env_logger;
use vec3::{Vec3, Point3};
use hittable::{HitRecord, Hittable};
use hittable_list::{HittableList};
use sphere::Sphere;
use color::{Color, write_color};
use ray::Ray;
use std::sync::Arc;
use camera::Camera;

fn main() -> io::Result<()> {
    env_logger::init();

    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i32 = 800;

    // World
    let mut world = HittableList::new();
    
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let cam: Camera = Camera::new(16.0/9.0, 400);
    cam.render(&world)
}
