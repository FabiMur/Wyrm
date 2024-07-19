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
    // Temporary hittable list to store the objects before creating the BVH
    let mut world = HittableList::new();

    // Texture inicialization
    let checker = CheckerTexture::new_from_colors(0.32, Color::new(0.3, 0.3, 0.3), Color::new(0.6,0.6,0.6));
    let material_ground = Lambertian::new_from_texture(Arc::new(checker));
    let earth_texture = ImageTexture::new("jupiter.jpeg");

    // Materials
    let left_red = Lambertian::new(Color::new(1.0, 0.2, 0.2));
    let back_green = Lambertian::new(Color::new(0.2, 1.0, 0.2));
    let right_blue = Lambertian::new(Color::new(0.2, 0.2, 1.0));
    let upper_orange = Lambertian::new(Color::new(1.0, 0.5, 0.0));
    let lower_teal = Lambertian::new(Color::new(0.2, 0.8, 0.8));

    // Quads
    world.add(Arc::new(Quad::new(Point3::new(-3.0, -2.0, 5.0), Vec3::new(0.0, 0.0, -4.0), Vec3::new(0.0, 4.0, 0.0), left_red.clone())));
    world.add(Arc::new(Quad::new(Point3::new(-2.0, -2.0, 0.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 4.0, 0.0), back_green.clone())));
    world.add(Arc::new(Quad::new(Point3::new(3.0, -2.0, 1.0), Vec3::new(0.0, 0.0, 4.0), Vec3::new(0.0, 4.0, 0.0), right_blue.clone())));
    world.add(Arc::new(Quad::new(Point3::new(-2.0, 3.0, 1.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 4.0), upper_orange.clone())));
    world.add(Arc::new(Quad::new(Point3::new(-2.0, -3.0, 5.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -4.0), lower_teal.clone())));

    // Build BVH from the temporary hittable object list and create a new hittable object list with it
    let bvh_node: Arc<BVHNode> = Arc::new(BVHNode::new(world.objects.clone(), 0, world.objects.len()));
    let mut new_world = HittableList::new();
    new_world.add(bvh_node);

    // Camera settings
    let aspect_ratio = 1.0;
    let image_width = 400;
    let vfov = 80.0;
    let lookfrom = Point3::new(0.0, 0.0, 9.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let defocus_angle: f64 = 0.0;
    let focus_dist: f64 = (lookfrom - lookat).length();

    let cam: Camera = Camera::new(aspect_ratio, image_width, vfov, lookfrom, lookat, vup, defocus_angle, focus_dist);

    // Render the scene
    cam.render(&new_world, "output.ppm");
}
