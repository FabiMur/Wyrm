use log::info;
use in_one_weekend::color::{Color, write_color};
use std::io::{self};
use env_logger;

fn main() -> io::Result<()> {
    env_logger::init();

    let image_width = 256;
    let image_height = 256;

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        info!("Scanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_color = Color::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.25,
            );

            write_color(&mut io::stdout(), &pixel_color)?;
        }
    }

    Ok(())
}
