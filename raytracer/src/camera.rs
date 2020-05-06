use crate::base::point3::Point3;
use crate::base::vec3::Vec3;
use crate::ray::Ray;

#[derive(Debug, Clone)]
pub struct Camera<T> {
    pub upper_left_corner: Point3<T>,
    pub horizontal: Vec3<T>,
    pub vertical: Vec3<T>,
    pub origin: Point3<T>,
}

impl Default for Camera<f64> {
    fn default() -> Self {
        Camera {
            upper_left_corner: Point3([-2.0, 1.0, 1.0].into()),
            horizontal: Vec3([4.0, 0.0, 0.0].into()),
            vertical: Vec3([0.0, -2.0, 0.0].into()),
            origin: Point3::default(),
        }
    }
}

impl Camera<f64> {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray<f64> {
        Ray {
            origin: self.origin.clone(),
            direction: (&self.upper_left_corner + &(&self.horizontal * u + &self.vertical * v))
                .vec_from(&self.origin),
        }
    }
}
