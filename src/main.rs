#![allow(unused)]

use std::{io::{self}, f64::consts::PI};

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
use materials::{Dielectric, DiffuseLight, Lambertian, Metal};
use utils::random_double;
use hittable::hittable_list::{HittableList};
use primitives::sphere::Sphere;
use camera::Camera;
use bvh::*;
use textures::*;
use external::load_ply;

fn main() {
    // Create the hittable list (world)
    let mut world = HittableList::new();

    // Define materials
    let light = DiffuseLight::new(Color::new(15.0, 15.0, 15.0));
    let white = Lambertian::new(Color::new(0.73, 0.73, 0.73));
    let gray = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    let green = Lambertian::new(Color::new(0.12, 0.45, 0.15));
    let red = Lambertian::new(Color::new(0.65, 0.05, 0.05));
    let blue = Lambertian::new(Color::new(0.05, 0.05, 0.65));
    let orange = Lambertian::new(Color::new(0.95, 0.65, 0.05));
    let glass = Dielectric::new(1.5);

    // Load triangles from a .ply file
    let ply_triangles = load_ply("models/cow.ply", glass.clone());
    for hittable in ply_triangles.objects {
        world.add(hittable);
    }



    // Floor
    world.add(Arc::new(Quad::new(
        Point3::new(-4.0, 0.0, -4.0),
        Vec3::new(0.0, 0.0, 8.0),
        Vec3::new(8.0, 0.0, 0.0),
        gray.clone(),
    )));

    // Ceiling
    world.add(Arc::new(Quad::new(
        Point3::new(-4.0, 4.0, -4.0),
        Vec3::new(0.0, 0.0, 8.0),
        Vec3::new(8.0, 0.0, 0.0),
        white.clone(),
    )));

    // Lamp
    world.add(Arc::new(Quad::new(
        Point3::new(3.0, 4.0, -2.0),
        Vec3::new(0.0, 0.0, 1.8),
        Vec3::new(-1.8, 0.0, 0.0),
        light.clone(),
    )));    

    // Back Wall
    world.add(Arc::new(Quad::new(
        Point3::new(-4.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        Vec3::new(8.0, 0.0, 0.0),
        blue.clone(),
    )));
    
    // Right Wall
    world.add(Arc::new(Quad::new(
        Point3::new(-4.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        Vec3::new(0.0, 0.0, 8.0),
        red.clone(),
    )));

    // Left Wall
    world.add(Arc::new(Quad::new(
        Point3::new(4.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        Vec3::new(0.0, 0.0, 8.0),
        green.clone(),
    )));
    
    // Camera settings
    let aspect_ratio = 2.0;
    let image_width = 1300;
    let vfov = 40.0;
    let lookfrom = Point3::new(0.0, 2.0, -8.5);
    let lookat = Point3::new(0.0, 2.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let focus_dist = (lookfrom - lookat).length();

    let cam = Camera::new(
        aspect_ratio,
        image_width,
        vfov,
        lookfrom,
        lookat,
        vup,
        defocus_angle,
        focus_dist,
    );

    // Build BVH from the temporary hittable object list and create a new hittable object list with it
    let bvh_node: Arc<BVHNode> = Arc::new(BVHNode::new(world.objects.clone()));
    let mut new_world = HittableList::new();
    new_world.add(bvh_node);

    // Render the scene
    cam.render(&new_world, "output.ppm");
}