use std::sync::Arc;

use criterion::{criterion_group, criterion_main, Criterion};

extern crate wyrm;

use wyrm::materials::{Lambertian, Metal, Dielectric};
use wyrm::primitives::*;
use wyrm::hittable::hittable_list::HittableList;
use wyrm::primitives::sphere::Sphere;
use wyrm::camera::Camera;
use wyrm::utils::random_double;

fn configure_criterion() -> Criterion {
    Criterion::default()
        .sample_size(10) // Número de muestras
        .measurement_time(std::time::Duration::from_secs(7200)) // Tiempo de medición por muestra
}

fn render_benchmark1() {
    env_logger::init();

    let mut world = HittableList::new();
    let material_ground = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, material_ground)));

    for i in -11..11 {
        for j in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(i as f64 + 0.9 * random_double(), 0.2, j as f64 + 0.9 * random_double());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material>;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = color::random();
                    sphere_material = Lambertian::new(albedo);
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));

                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = color::random();
                    sphere_material = Metal::new(albedo, 0.5);
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));

                } else {
                    // glass
                    sphere_material = Dielectric::new(1.5);
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    // Camera settings
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i32 = 400;
    let vfov: f64 = 20.0;
    let lookfrom: Point3 = Point3::new(-2.0, 2.0, 1.0);
    let lookat: Point3 = Point3::new(0.0, 0.0, -1.0);
    let vup: Point3 = Point3::new(0.0, 1.0, 0.0);
    let defocus_angle: f64 = 0.6;
    let focus_dist: f64 = 10.0;

    // Materials
    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.50);
    let material_bubble = Dielectric::new(1.0/1.50);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

    // World

    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, material_center)));
    world.add(Arc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.add(Arc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.4, material_bubble)));
    world.add(Arc::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right)));

    let cam: Camera = Camera::new(aspect_ratio, image_width, vfov, lookfrom, lookat, vup,
        defocus_angle ,focus_dist);

    let _result = cam.render(&world, "benchmark1.ppm");
}

fn render(c: &mut Criterion) {
    c.bench_function("Render benchmark 1", |b| b.iter(|| render_benchmark1()));
}

criterion_group! {
    name = benches;
    config = configure_criterion();
    targets = render
}
criterion_main!(benches);