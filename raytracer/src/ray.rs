use std::ops::{Mul, Add};

use crate::base::color::Color;
use crate::base::point3::Point3;
use crate::base::vec3::{Vec3, Vec3Operations};
use crate::base::XYZ;
use crate::hittable::Hit;
use crate::materials::Scatter;

#[derive(Debug, Default, Clone)]
pub struct Ray<T> {
    pub origin: Point3<T>,
    pub direction: Vec3<T>,
}
impl<T> Ray<T>
where T: Mul<T, Output = T> + Add<T, Output = T> + Copy
{
    pub fn at(&self, t: T) -> Point3<T> {
        &self.origin + &(&self.direction * t)
    }
}

pub fn ray_color<'a, T>(ray: &Ray<f64>, world: &'a T, depth: u16) -> Color<f64>
where
    &'a T: Hit<f64>,
{
    if depth == 0 {
        return Color::default();
    }

    match world.hit(&ray, 0.0001, f64::INFINITY) {
        Some(rec) => match rec.material.scatter(ray, &rec) {
            Some((scattered, attenuation)) => {
                attenuation * ray_color(&scattered, &world, depth - 1)
            }
            None => Color::default(),
        },
        None => {
            let t = 0.5 * (ray.direction.unit().y() + 1.0);
            Color([1.0, 1.0, 1.0].into()) * (1.0 - t) + Color([0.5, 0.7, 1.0].into()) * t
        }
    }
}
