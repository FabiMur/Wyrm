#![allow(unused)]

use std::{io::{self}, f64::consts::PI};
use env_logger;

mod camera;
mod hittable;
mod utils;
mod primitives;
mod materials;
mod bvh;
mod textures;
mod external;

use std::sync::Arc;
use primitives::*;
use materials::{Lambertian, Metal, Dielectric, MaterialArcWrapper};
use utils::random_double;
use hittable::hittable_list::{HittableList};
use primitives::sphere::Sphere;
use camera::Camera;
use bvh::*;
use textures::*;

fn main() {
    let mut world = HittableList::new();

    // Texture inicialization
    let checker = CheckerTexture::new_from_colors(0.32, Color::new(0.3, 0.3, 0.3), Color::new(0.6,0.6,0.6));
    let material_ground = Lambertian::new_from_texture(Arc::new(checker));
    let earth_texture = ImageTexture::new("earthSurface.jpg");


    let material_left = Dielectric::new(1.5);
    let material_center = Lambertian::new_from_texture(Arc::new(earth_texture));
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 0.0);

    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Arc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material_center)));
    world.add(Arc::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right)));

    // Build BVH from the world
    let bvh_node: Arc<BVHNode> = Arc::new(BVHNode::new(world.objects.clone(), 0, world.objects.len()));
    let mut new_world = HittableList::new();
    new_world.add(bvh_node);

    // Camera settings
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i32 = 400;
    let vfov: f64 = 20.0;
    let lookfrom: Point3 = Point3::new(0.0, 0.0, -7.0);
    let lookat: Point3 = Point3::new(0.0, 0.0, -1.0);
    let vup: Point3 = Point3::new(0.0, 1.0, 0.0);
    let defocus_angle: f64 = 0.0;
    let focus_dist: f64 = (lookfrom - lookat).length();

    let cam: Camera = Camera::new(aspect_ratio, image_width, vfov, lookfrom, lookat, vup, defocus_angle, focus_dist);

    // Render the scene
    cam.render(&new_world, "output.ppm");
}
