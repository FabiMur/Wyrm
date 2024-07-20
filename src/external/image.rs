extern crate image;

use std::env;
use std::path::Path;

#[derive(Default)]
pub struct Image {
    width: u32,
    height: u32,
    bdata: Option<Vec<u8>>,
    bytes_per_pixel: u32,
    bytes_per_scanline: u32,
}

impl Image {
    pub fn new() -> Self {
        Image {
            width: 0,
            height: 0,
            bdata: None,
            bytes_per_pixel: 3,
            bytes_per_scanline: 0,
        }
    }

    pub fn from_file(image_filename: &str) -> Self {
        let mut img = Image::new();
        let filename = String::from(image_filename);
        let imagedir = env::var("RTW_IMAGES").ok();

        if let Some(dir) = imagedir {
            if img.load(&format!("{}/{}", dir, image_filename)) {
                return img;
            }
        }

        if img.load(&filename) ||
           img.load(&format!("images/{}", filename)) ||
           img.load(&format!("../images/{}", filename)) ||
           img.load(&format!("../../images/{}", filename)) ||
           img.load(&format!("../../../images/{}", filename)) ||
           img.load(&format!("../../../../images/{}", filename)) ||
           img.load(&format!("../../../../../images/{}", filename)) ||
           img.load(&format!("../../../../../../images/{}", filename)) {
            return img;
        }

        eprintln!("ERROR: Could not load image file '{}'.", image_filename);
        img
    }

    fn load(&mut self, filename: &str) -> bool {
        let img = image::open(&Path::new(filename));
        match img {
            Ok(img) => {
                self.width = img.width();
                self.height = img.height();
                self.bytes_per_scanline = self.width * self.bytes_per_pixel;
                self.bdata = Some(img.to_rgb8().into_raw());
                true
            }
            Err(_) => false,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn pixel_data(&self, x: u32, y: u32) -> &[u8] {
        static MAGENTA: [u8; 3] = [255, 0, 255];
        if self.bdata.is_none() {
            return &MAGENTA;
        }

        let x = Self::clamp(x, 0, self.width);
        let y = Self::clamp(y, 0, self.height);
        let index = (y * self.bytes_per_scanline + x * self.bytes_per_pixel) as usize;
        &self.bdata.as_ref().unwrap()[index..index + 3]
    }

    fn clamp(x: u32, low: u32, high: u32) -> u32 {
        if x < low {
            low
        } else if x < high {
            x
        } else {
            high - 1
        }
    }
}
