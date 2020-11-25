use image::RgbImage;
use indicatif::{ProgressBar, ProgressStyle};
use rand::prelude::*;
use rayon::prelude::*;

use raytracer::*;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1920;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 1000;
    let max_depth = 50;

    let look_from = Point3([13.0, 2.0, -3.0].into());
    let look_at = Point3([0.0, 0.0, 0.0].into());
    let up = Vec3([0.0, 1.0, 0.0].into());
    let vertical_fov = 20.0;
    let aperture = 0.1;
    let dist_to_focus = 10.0;

    let mut img = RgbImage::new(image_width, image_height);
    let world = random_scene();

    let cam = Camera::new(
        look_from,
        look_at,
        up,
        vertical_fov,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );
    let progress_bar = ProgressBar::new(image_height as u64);
    let pb_style = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} ({eta})")
        .progress_chars("##-");
    progress_bar.set_style(pb_style);

    img.enumerate_rows_mut().par_bridge().into_par_iter().for_each(
        |(_, chunk)| {
            let mut rng = thread_rng();
            chunk.for_each(
            |(x, y, pixel)| {
                let mut color = Color::default();
                for _ in 0..samples_per_pixel {
                    let u = (x as f64 + rng.gen_range(0.0, 1.0)) / image_width as f64;
                    let v = (y as f64 + rng.gen_range(0.0, 1.0)) / image_height as f64;

                    let ray = cam.get_ray(u, v);
                    color += &ray_color(&ray, &world, max_depth);
                }
                *pixel = color.as_rgb(samples_per_pixel);
            });
            progress_bar.inc(1);
        }
    );

    img.save("image.png").unwrap();
}

fn random_scene() -> HittableVec<f64> {
    let mut world = HittableVec {
        objects: Vec::new(),
    };
    let mut spheres = HittableVec {
        objects: Vec::new(),
    };
    spheres.push(Hittable::Sphere(Sphere {
        center: Point3([0.0, -1000.0, 0.0].into()),
        radius: 1000.0,
        material: Materials::Lambertian(Lambertian {
            albedo: Color([0.5, 0.5, 0.5].into()),
        }),
    }));

    spheres.push(Hittable::Sphere(Sphere {
        center: Point3([0.0, 1.0, 0.0].into()),
        radius: 1.0,
        material: Materials::Dielectric(Dielectric { ref_idx: 1.5 }),
    }));
    spheres.push(Hittable::Sphere(Sphere {
        center: Point3([-4.0, 1.0, 0.0].into()),
        radius: 1.0,
        material: Materials::Lambertian(Lambertian {
            albedo: Color([0.4, 0.2, 0.1].into()),
        }),
    }));
    spheres.push(Hittable::Sphere(Sphere {
        center: Point3([4.0, 1.0, 0.0].into()),
        radius: 1.0,
        material: Materials::Metal(Metal {
            albedo: Color([0.8, 0.6, 0.5].into()),
            fuzz: 0.0,
        }),
    }));

    let mut rng = thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_material = rng.gen_range(0.0, 1.0);
            let center = Point3(
                [
                    a as f64 + 0.9 * rng.gen_range(0.0, 1.0),
                    0.2,
                    b as f64 + 0.9 * rng.gen_range(0.0, 1.0),
                ]
                .into(),
            );
            if choose_material < 0.8 {
                //diffuse
                let albedo = Color::random(0.0, 1.0);
                spheres.push(Hittable::Sphere(Sphere {
                    center,
                    radius: 0.2,
                    material: Materials::Lambertian(Lambertian { albedo }),
                }));
            } else if choose_material < 0.95 {
                //metal
                let albedo = Color::random(0.5, 1.0);
                let fuzz = rng.gen_range(0.0, 0.5);
                spheres.push(Hittable::Sphere(Sphere {
                    center,
                    radius: 0.2,
                    material: Materials::Metal(Metal { albedo, fuzz }),
                }));
            } else {
                //glass
                spheres.push(Hittable::Sphere(Sphere {
                    center,
                    radius: 0.2,
                    material: Materials::Dielectric(Dielectric { ref_idx: 1.5 }),
                }));
            }
        }
    }

    world.push(Hittable::HittableVec(spheres));

    world
}
