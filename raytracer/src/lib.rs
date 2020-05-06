mod base;
mod camera;
mod hittable;
mod materials;
mod ray;
mod utils;

pub use crate::base::color::Color;
pub use crate::base::point3::Point3;
pub use crate::base::vec3::{Vec3, Vec3Operations};
pub use crate::camera::Camera;
pub use crate::hittable::{Hittable, HittableVec, Sphere};
pub use crate::materials::{Lambertian, Materials, Metal};
pub use crate::ray::{ray_color, Ray};
