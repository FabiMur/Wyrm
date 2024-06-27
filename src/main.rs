#![allow(unused)]

use std::io::{self};
use env_logger;

mod camera;
mod hittable;
mod utils;
mod primitives;

use primitives::vec3::Point3;
use hittable::hittable_list::{HittableList};
use primitives::sphere::Sphere;
use std::sync::Arc;
use camera::Camera;

fn main() -> io::Result<()> {
    env_logger::init();

    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i32 = 720;

    // World
    let mut world = HittableList::new();
    
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let cam: Camera = Camera::new(aspect_ratio, image_width);
    cam.render(&world, "output.ppm")
}
