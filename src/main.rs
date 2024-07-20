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

fn main() {

    // Create the hittable list (world)
    let mut world = HittableList::new();

    // Define materials
    let red = Lambertian::new(Color::new(0.65, 0.05, 0.05));
    let white = Lambertian::new(Color::new(0.73, 0.73, 0.73));
    let green = Lambertian::new(Color::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::new(Color::new(15.0, 15.0, 15.0));
    let glass = Dielectric::new(1.5);
    let metal = Metal::new(Color::new(0.8, 0.8, 0.8), 0.3);
    let earth_surface = Arc::new(ImageTexture::new("earthSurface.jpg"));
    let earth_surface = Lambertian::new_from_texture(earth_surface);

    // Add quads for the Cornell Box
    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(213.0, 554.0, 227.0), // Adjusted position to center the larger light
        Vec3::new(219.0, 0.0, 0.0), // Increased length to 219
        Vec3::new(0.0, 0.0, 211.0), // Increased length to 211
        light.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    // Add spheres
    world.add(Arc::new(Sphere::new(
        Point3::new(190.0, 90.0, 190.0),
        90.0,
        glass.clone(),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(370.0, 90.0, 370.0),
        90.0,
        earth_surface.clone(),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(200.0, 90.0, 350.0),
        90.0,
        metal.clone(),
    )));
    

    // Camera settings
    let aspect_ratio = 1.0;
    let image_width = 1080;
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
    let bvh_node: Arc<BVHNode> = Arc::new(BVHNode::new(world.objects.clone(), 0, world.objects.len()));
    let mut new_world = HittableList::new();
    new_world.add(bvh_node);

    // Render the scene
    cam.render(&new_world, "output.ppm");
}
