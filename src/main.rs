#![allow(unused)]

mod vec3;
mod color;
mod ray;
mod utils;
mod hittable;
mod hittable_list;
mod sphere;

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

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::default();
    if world.hit(r, 0.001, INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    //Color::new(0.0, 0.0, 0.0)
}


fn main() -> io::Result<()> {
    env_logger::init();

    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i32 = 400;

    // Calculate the image height, and ensure it's at least 1
    let image_height: i32 = max ((image_width as f64 / aspect_ratio) as i32, 1);

    // World
    let mut world = HittableList::new();
    
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
    //world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
    //world.add(Arc::new(Sphere::new(Point3::new(0.0, 100.5, -1.0), 100.0)));

    // Camera
    let focal_length: f64 = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center: Point3 = Point3::new(0.0,0.0,0.0);

    // viewport_u represents the "horizontal component" of the wievport.
    // In the viewport plane it goes to the right.
    let viewport_u: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);

    // viewport_u represents the "vertical component" of the wievport.
    // In the viewport plane it goes downwards.
    let viewport_v: Vec3 = Vec3::new(0.0, -viewport_height, 0.0);

    // The horizontal and vertical vectors ("distance") from pixel to pixel.
    let pixel_delta_u: Vec3 = viewport_u / image_width as f64;
    let pixel_delta_v: Vec3 = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel moving (focal_length) units towards the camera
    // then moving half of the viewport width to the left and half of the viewport height upwards.
    let viewport_upper_left: Point3 = camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u/2.0 - viewport_v/2.0;
    let pixel00_loc: Point3 = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);


    
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height) {
        info!("Scanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            // Calculate the center of the pixel center moving i times to the right and 
            // j times downwards using the pixel_delta vectors.
            let pixel_center: Vec3 = pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            // Ray from the camera towards the pixel center
            let ray_direction: Vec3 = pixel_center - camera_center;
            let r: Ray = Ray::new(camera_center, ray_direction);

            let pixel_color: Color = ray_color(&r, &world);

            write_color(&mut io::stdout(), &pixel_color)?;
        }
    }

    Ok(())
}
