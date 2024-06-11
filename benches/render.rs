#![allow(unused)]

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, Throughput, PlotConfiguration, AxisScale, BenchmarkGroup};
use rand::Rng;
use fastrand;

use in_one_weekend::vec3::{Vec3, Point3}; // Asegúrate de reemplazar `your_crate_name` con el nombre real de tu crate.
use in_one_weekend::color::{Color, write_color};
use in_one_weekend::ray::Ray;
use in_one_weekend::utils::INFINITY;
use in_one_weekend::hittable::{HitRecord, Hittable};
use in_one_weekend::hittable_list::HittableList;
use in_one_weekend::sphere::Sphere;
use in_one_weekend::camera::Camera;
use std::sync::Arc;

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
    cam.render(&world, "output.ppm");
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