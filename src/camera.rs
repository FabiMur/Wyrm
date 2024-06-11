use crate::utils::{INFINITY, random_double};
use crate::vec3::{Point3, Vec3, Color};
use crate::ray::{Ray};
use crate::hittable::{HitRecord, Hittable};
use crate::color::write_color;
use std::cmp::max;
use std::io::{self};
use log::info;
use crate::interval::Interval;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub image_height: i32,
    pub center: Point3,
    pub pixel00_loc: Point3,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
    pub samples_per_pixel: i32,
    pub pixel_samples_scale: f64,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32) -> Self {
        // Calculate the image height, and ensure it's at least 1
        let image_height = max((image_width as f64 / aspect_ratio) as i32, 1);

        let center = Point3::new(0.0, 0.0, 0.0);

        // Viewport dimensions
        let focal_length: f64 = 1.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = viewport_height * aspect_ratio;

        // viewport_u represents the "horizontal component" of the viewport.
        // In the viewport plane it goes to the right.
        let viewport_u: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);

        // viewport_v represents the "vertical component" of the viewport.
        // In the viewport plane it goes downwards.
        let viewport_v: Vec3 = Vec3::new(0.0, -viewport_height, 0.0);

        // The horizontal and vertical vectors ("distance") from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel moving (focal_length) units towards the camera
        // then moving half of the viewport width to the left and half of the viewport height upwards.
        let viewport_upper_left: Point3 = center
            - Vec3::new(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Samples per pixel used for antialiasing
        let samples_per_pixel: i32 = 100;
        let pixel_samples_scale: f64 = 1.0 / samples_per_pixel as f64;

        Self {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            pixel_samples_scale,
        }
    }

    pub fn render(&self, world: &dyn Hittable) -> io::Result<()> {
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            info!("Scanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color: Vec3 = Vec3::new(0.0, 0.0, 0.0);
                for _sample in 0..self.samples_per_pixel {
                    let r: Ray = self.get_ray(i,j);
                    pixel_color = pixel_color + ray_color(&r, world);
                }

                write_color(&mut io::stdout(), &(self.pixel_samples_scale * pixel_color))?;
            }
        }
        Ok(())
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset: Vec3 = sample_square();
        //println!("{:?}", offset);
        let pixel_sample: Vec3 = self.pixel00_loc 
            + ((i as f64 + offset.x) * self.pixel_delta_u) 
            + ((j as f64 + offset.y) * self.pixel_delta_v);
        
        let ray_origin: Point3 = self.center;
        let ray_direction: Vec3 = pixel_sample - ray_origin;
    
        return Ray::new(ray_origin, ray_direction);
    }

}

fn sample_square() -> Vec3 {
    Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
}

pub fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::default();
    if world.hit(r, Interval::new(0.0001, INFINITY), &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}

