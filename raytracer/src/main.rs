use image::RgbImage;
use indicatif::ProgressBar;
use num_cpus;
use rand::prelude::*;
use rayon::ThreadPoolBuilder;

use raytracer::*;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 384;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let max_depth = 50u8;
    let pool = ThreadPoolBuilder::new()
        .num_threads(num_cpus::get())
        .build()
        .unwrap();

    let mut img = RgbImage::new(image_width, image_height);
    let mut world = HittableVec {
        objects: Vec::new(),
    };
    let mut spheres = HittableVec {
        objects: Vec::new(),
    };
    spheres.push(Hittable::Sphere(Sphere {
        center: Point3([0.0, 0.0, 1.0].into()),
        radius: 0.5,
        material: Materials::Lambertian(Lambertian {
            albedo: Color([0.7, 0.3, 0.3].into()),
        }),
    }));
    spheres.push(Hittable::Sphere(Sphere {
        center: Point3([0.0, -100.5, 1.0].into()),
        radius: 100.0,
        material: Materials::Lambertian(Lambertian {
            albedo: Color([0.8, 0.8, 0.0].into()),
        }),
    }));

    spheres.push(Hittable::Sphere(Sphere {
        center: Point3([1.0, 0.0, 1.0].into()),
        radius: 0.5,
        material: Materials::Metal(Metal {
            albedo: Color([0.8, 0.6, 0.2].into()),
            fuzz: 0.1,
        }),
    }));
    spheres.push(Hittable::Sphere(Sphere {
        center: Point3([-1.0, 0.0, 1.0].into()),
        radius: 0.5,
        material: Materials::Metal(Metal {
            albedo: Color([0.8, 0.8, 0.8].into()),
            fuzz: 0.5,
        }),
    }));

    world.push(Hittable::HittableVec(spheres));

    let progress_bar = ProgressBar::new(image_height as u64);
    let cam = Camera::default();
    let rows = img.rows_mut().collect::<Vec<_>>();
    pool.scope(|scope| {
        let cam_ref = &cam;
        let world_ref = &world;
        let progress_bar_ref = &progress_bar;
        for (y, chunk) in rows.into_iter().enumerate() {
            scope.spawn(move |_| {
                for (x, pixel) in chunk.enumerate() {
                    let mut rng = thread_rng();
                    let mut color = Color::default();

                    for _ in 0..samples_per_pixel {
                        let u = (x as f64 + rng.gen_range(0.0, 1.0)) / image_width as f64;
                        let v = (y as f64 + rng.gen_range(0.0, 1.0)) / image_height as f64;

                        let ray = cam_ref.get_ray(u, v);
                        color += &ray_color(&ray, world_ref, max_depth);
                    }
                    *pixel = color.as_rgb(samples_per_pixel);
                }
                progress_bar_ref.inc(1);
            });
        }
    });

    img.save("image.png").unwrap();
}
