#![allow(unused)]

use std::{io::{self}, f64::consts::PI};
use std::sync::Arc;

mod camera;
mod hittable;
mod utils;
mod primitives;
mod materials;
mod bvh;
mod textures;
mod external;

use primitives::*;
use materials::*;
use utils::random_double;
use camera::Camera;
use hittable::*;
use primitives::*;
use bvh::*;
use textures::*;
use external::load_ply;

fn main() {
    // Create the hittable list (world)
    let mut world = HittableList::new();

    // Define materials
    let light = DiffuseLight::new(Color::new(18.0, 18.0, 18.0));
    let white = Lambertian::new(Color::new(0.73, 0.73, 0.73));
    let green = Lambertian::new(Color::new(0.12, 0.45, 0.15));
    let red = Lambertian::new(Color::new(0.65, 0.05, 0.05));
    let blue = Lambertian::new(Color::new(0.05, 0.05, 0.65));
    let orange = Lambertian::new(Color::new(0.95, 0.65, 0.05));
    let glass = Dielectric::new(1.5);

    // Left Wall
    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green.clone(),
    )));

    // Right Wall
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red.clone(),
    )));

    // Ceiling Light
    world.add(Arc::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light.clone(),
    )));

    // Background Wazll
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        blue.clone(),
    )));

    // Floor
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));

    // Ceiling
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 555.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));

    // Big Box
    let mut box_1 = Arc::new(Quad::new_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        glass.clone(),
    ));
    let box_1 =  Arc::new(RotationY::new(box_1, 15.0));
    let box_1 =  Arc::new(Translation::new(box_1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(box_1);
    

    // Small Box
    let mut box_2 = Arc::new(Quad::new_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    ));
    let box_2 =  Arc::new(RotationY::new(box_2, -18.0));
    let box_2 =  Arc::new(Translation::new(box_2, Vec3::new(130.0, 0.0, 65.0)));
    world.add(box_2);
    
    // Camera settings
    let aspect_ratio = 1.0;
    let image_width = 1440;
    let vfov = 40.0;
    let lookfrom = Point3::new(278.0, 278.0, -800.0);
    let lookat = Point3::new(278.0, 278.0, 0.0);
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
