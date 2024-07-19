use crate::primitives::*;
use crate::textures::*;
use crate::external::image::Image;

pub struct ImageTexture {
    pub image: Image
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        let image:Image = Image::from_file(filename);
        Self { image }
    }
}

impl Texture for ImageTexture {
fn value(&self, mut u: f64, mut v: f64, _p: &Point3) -> Color {
        if self.image.height() <= 0 {
            return Color::new(1.0, 0.0, 0.0); // Return red for debugging
        }

        // Clamp input texture coordinates to [0,1] x [1,0]
        u = Interval::new(0.0, 1.0).clamp(u);
        v = 1.0 - Interval::new(0.0, 1.0).clamp(v); // Flip V to image coordinates

        let mut i = (u * self.image.width() as f64) as u32;
        let mut j = (v * self.image.height() as f64) as u32;

        // Clamp integer mapping, since actual coordinates should be less than 1.0
        i = i.min(self.image.width() - 1);
        j = j.min(self.image.height() - 1);

        let pixel = self.image.pixel_data(i, j);

        let color_scale = 1.0 / 255.0;
        Color::new(
            color_scale * pixel[0] as f64,
            color_scale * pixel[1] as f64,
            color_scale * pixel[2] as f64,
        )
    }
}
