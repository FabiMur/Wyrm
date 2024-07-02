use std::sync::Arc;

use criterion::{criterion_group, criterion_main, Criterion};

extern crate in_one_weekend;

use in_one_weekend::materials::{Lambertian, Metal, Dielectric};
use in_one_weekend::primitives::*; // Asegúrate de reemplazar `your_crate_name` con el nombre real de tu crate.
use in_one_weekend::hittable::hittable_list::{HittableList};
use in_one_weekend::primitives::sphere::Sphere;
use in_one_weekend::camera::Camera;

fn configure_criterion() -> Criterion {
    Criterion::default()
        .sample_size(10) // Número de muestras
        .measurement_time(std::time::Duration::from_secs(100)) // Tiempo de medición por muestra
}

fn render_benchmark1() {
    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: i32 = 720;
    // Materials
    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0)); // Create a red Lambertian material
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5)); // Create a red Lambertian material
    let material_left = Dielectric::new(1.5); // Create a red Lambertian material
    let material_bubble = Dielectric::new(1.0/1.5); // Create a red Lambertian material
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0); // Create a red Lambertian material

    // World
    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground.clone())));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, material_center.clone())));
    world.add(Arc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left.clone())));
    world.add(Arc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.4, material_bubble.clone())));
    world.add(Arc::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right.clone())));
    
    let cam: Camera = Camera::new(aspect_ratio, image_width);
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