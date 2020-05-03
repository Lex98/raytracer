use image::RgbImage;
use indicatif::ProgressIterator;
use rand::prelude::*;

use raytracer::*;

fn main() {
    let mut rng = thread_rng();
    let image_width = 200;
    let image_height = 100;
    let samples_per_pixel = 100;

    let mut img = RgbImage::new(image_width, image_height);

    let mut world = HittableVec {
        objects: Vec::new(),
    };
    world.push(Hittable::Sphere(Sphere {
        center: Point3([0.0, 0.0, 1.0].into()),
        radius: 0.5,
    }));
    world.push(Hittable::Sphere(Sphere {
        center: Point3([0.0, -100.5, 1.0].into()),
        radius: 100.0,
    }));

    let cam = Camera::default();

    for j in (0..image_height).progress() {
        for i in 0..image_width {
            let mut color = Color::default();

            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen_range(0.0, 1.0)) / image_width as f64;
                let v = (j as f64 + rng.gen_range(0.0, 1.0)) / image_height as f64;

                let ray = cam.get_ray(u, v);
                color += &ray_color(&ray, &world);
            }
            img.put_pixel(i, j, color.as_rgb(samples_per_pixel))
        }
    }
    img.save("image.png").unwrap();
}
