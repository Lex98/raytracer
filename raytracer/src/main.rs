use indicatif::ProgressIterator;
use std::io;

use raytracer::*;

fn main() -> io::Result<()> {
    let image_height = 100;
    let image_width = 200;

    println!("P3\n{} {}\n255", image_width, image_height);

    for i in (0..image_height).rev().progress() {
        for j in 0..image_width {
            let color = Color([
                i as f64 / image_height as f64, 
                j as f64 / image_width as f64,
                0.2
            ].into());

            color.write(&mut io::stdout())?;
        }
    }
    Ok(())
}
