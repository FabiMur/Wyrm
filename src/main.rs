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

    let red = Lambertian::new(Color::new(1.0, 0.0, 0.0));
    let green = Lambertian::new(Color::new(0.0, 1.0, 0.0));
    let blue = Lambertian::new(Color::new(0.0, 0.0, 1.0));
    let pourple = Lambertian::new(Color::new(5.0, 0.0, 7.0));
    let white = Lambertian::new(Color::new(1.0, 1.0, 1.0));
    let spec = Specular::new();
    let refrac = Refractive::new(1.5);

    let material_matt_red = Arc::new(Material::new(
        red.clone(), 
        spec.clone(), 
        refrac.clone(),
        None, 
        1.0, 0.0, 0.0, 0.0)
    );

    let material_matt_green = Arc::new(Material::new(
        green.clone(), 
        spec.clone(), 
        refrac.clone(),
        None, 
        1.0, 0.0, 0.0, 0.0)
    );

    let material_matt_blue = Arc::new(Material::new(
        blue.clone(), 
        spec.clone(), 
        refrac.clone(),
        None, 
        1.0, 0.0, 0.0, 0.0)
    );

    let material_matt_white = Arc::new(Material::new(
        white.clone(), 
        spec.clone(), 
        refrac.clone(),
        None, 
        1.0, 0.0, 0.0, 0.0)
    );

    let material_box = Arc::new(Material::new(
        pourple.clone(), 
        spec.clone(), 
        refrac.clone(),
        None, 
        0.0, 0.0, 1.0, 0.0)
    );
    
    let material_light = Arc::new(Material::new(
        white.clone(), 
        spec.clone(), 
        refrac.clone(),
        Some(Color::new(10.0, 10.0, 10.0)), 
        1.0, 0.0, 0.0, 0.0)
    );

    // Left Wall
    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        material_matt_green.clone(),
    )));

    // Right Wall
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        material_matt_red.clone(),
    )));

    // Ceiling Light
    world.add(Arc::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        material_light.clone(),
    )));

    // Background Wall
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        material_matt_blue.clone(),
    )));

    // Floor
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        material_matt_white.clone(),
    )));

    // Ceiling
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 555.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        material_matt_white.clone(),
    )));

    // Big Box
    let mut box_1 = Arc::new(Quad::new_box(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        material_box.clone(),
    ));
    let box_1 =  Arc::new(RotationY::new(box_1, 15.0));
    let box_1 =  Arc::new(Translation::new(box_1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(box_1);
    
    
    // Camera settings
    let aspect_ratio = 1.0;
    let image_width = 500;
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
