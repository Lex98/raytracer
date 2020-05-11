use rand::prelude::*;

use crate::base::point3::Point3;
use crate::base::vec3::{Vec3, Vec3Operations};
use crate::base::XYZ;
use crate::ray::Ray;

#[derive(Debug, Clone, Default)]
pub struct Camera<T> {
    upper_left_corner: Point3<T>,
    horizontal: Vec3<T>,
    vertical: Vec3<T>,
    pub origin: Point3<T>,
    u: Vec3<T>,
    v: Vec3<T>,
    w: Vec3<T>,
    lens_radius: T,
}

impl Camera<f64> {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray<f64> {
        let mut rng = thread_rng();
        let rd = Vec3([rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0), 0.0].into()).unit()
            * self.lens_radius;
        let offset = &self.u * *rd.x() + &self.v * *rd.y();
        Ray {
            origin: &self.origin + &offset,
            direction: (&self.upper_left_corner + &(&self.horizontal * u + &self.vertical * v))
                .vec_from(&self.origin)
                - offset,
        }
    }

    pub fn new(
        look_from: Point3<f64>,
        look_at: Point3<f64>,
        up: Vec3<f64>,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let lens_radius = aperture / 2.0;

        let theta = vfov.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = half_height * aspect_ratio;

        let w = look_at.vec_from(&look_from).unit();
        let u = up.cross(&w).unit();
        let v = w.cross(&u);

        let upper_left_corner = &look_from
            - &(&u * focus_dist * half_width - &v * focus_dist * half_height - &w * focus_dist);
        let horizontal = &u * half_width * focus_dist * 2.0;
        let vertical = &v * half_height * focus_dist * -2.0;

        Camera {
            upper_left_corner,
            horizontal,
            vertical,
            origin: look_from,
            u,
            v,
            w,
            lens_radius,
        }
    }
}
