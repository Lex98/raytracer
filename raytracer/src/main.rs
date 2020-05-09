use image::RgbImage;
use indicatif::{ProgressBar, ProgressStyle};
use num_cpus;
use rand::prelude::*;
use rayon::ThreadPoolBuilder;

use raytracer::*;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1920;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let max_depth = 50u16;
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
        center: Point3([0.0, -100.5, 1.0].into()),
        radius: 100.0,
        material: Materials::Lambertian(Lambertian {
            albedo: Color([0.8, 0.8, 0.0].into()),
        }),
    }));

    spheres.push(Hittable::Sphere(Sphere {
        center: Point3([0.0, 0.0, 1.0].into()),
        radius: 0.5,
        material: Materials::Lambertian(Lambertian {
            albedo: Color([0.7, 0.3, 0.3].into())
        }),
    }));
    spheres.push(Hittable::Sphere(Sphere {
        center: Point3([-1.0, 0.0, 1.0].into()),
        radius: 0.5,
        material: Materials::Dielectric(Dielectric {
            ref_idx: 1.5,
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

    world.push(Hittable::HittableVec(spheres));

    let cam = Camera::default();
    let rows = img.rows_mut().collect::<Vec<_>>();
    let progress_bar = ProgressBar::new(rows.len() as u64);
    let pb_style = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} ({eta})")
        .progress_chars("##-");
    progress_bar.set_style(pb_style);
    pool.scope(|scope| {
        let cam_ref = &cam;
        let world_ref = &world;
        let progress_bar_ref = &progress_bar;
        for (y, chunk) in rows.into_iter().enumerate() {
            scope.spawn(move |_| {
                let mut rng = thread_rng();
                for (x, pixel) in chunk.enumerate() {
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
        };
    });

    img.save("image.png").unwrap();
}
