use image::RgbImage;
use indicatif::ProgressIterator;

use raytracer::*;

fn main() {
    let image_width = 200;
    let image_height = 100;

    let mut img = RgbImage::new(image_width, image_height);

    let upper_left_corner = Point3([-2., 1., 1.].into());
    let horizontal = Vec3([4., 0., 0.].into());
    let vertical = Vec3([0., -2., 0.].into());

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

    for j in (0..image_height).progress() {
        for i in 0..image_width {
            let u = i as f64 / image_width as f64;
            let v = j as f64 / image_height as f64;

            let ray = Ray {
                origin: Point3::default(),
                direction: (&upper_left_corner + &(&horizontal * u + &vertical * v))
                    .vec_from(&Point3::default()),
            };
            let color = ray_color(&ray, &world);
            img.put_pixel(i, j, color.into())
        }
    }
    img.save("image.png").unwrap();
}
