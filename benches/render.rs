use std::result;
use std::sync::Arc;

use criterion::{criterion_group, criterion_main, Criterion};

extern crate in_one_weekend;

use in_one_weekend::primitives::vec3::Point3; // Asegúrate de reemplazar `your_crate_name` con el nombre real de tu crate.
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

    // World
    let mut world = HittableList::new();
    
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let cam: Camera = Camera::new(aspect_ratio, image_width);
    let _result = cam.render(&world, "output.ppm");
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