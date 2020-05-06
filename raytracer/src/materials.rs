use crate::base::color::Color;
use crate::base::vec3::{Vec3, Vec3Operations};
use crate::hittable::HitRecord;
use crate::ray::Ray;

#[derive(Debug, Clone)]
pub struct Lambertian<T> {
    pub albedo: Color<T>,
}

#[derive(Debug, Clone)]
pub struct Metal<T> {
    pub albedo: Color<T>,
    pub fuzz: T,
}

#[derive(Debug, Clone)]
pub enum Materials<T> {
    Lambertian(Lambertian<T>),
    Metal(Metal<T>),
}

pub trait Scatter<T> {
    fn scatter(&self, ray_in: &Ray<T>, hit_record: &HitRecord<T>) -> Option<(Ray<T>, Color<T>)>;
}

impl Scatter<f64> for Lambertian<f64> {
    fn scatter(
        &self,
        _ray_in: &Ray<f64>,
        hit_record: &HitRecord<f64>,
    ) -> Option<(Ray<f64>, Color<f64>)> {
        let scatter_direction = &hit_record.normal + &Vec3::random_unit();
        Some((
            Ray {
                origin: hit_record.point.clone(),
                direction: scatter_direction,
            },
            self.albedo.clone(),
        ))
    }
}

impl Scatter<f64> for Metal<f64> {
    fn scatter(
        &self,
        ray_in: &Ray<f64>,
        hit_record: &HitRecord<f64>,
    ) -> Option<(Ray<f64>, Color<f64>)> {
        let reflected = ray_in.direction.unit_vector().reflect(&hit_record.normal);
        let scattered = Ray {
            origin: hit_record.point.clone(),
            direction: reflected + Vec3::random_unit() * self.fuzz,
        };
        if scattered.direction.dot(&hit_record.normal) > 0.0 {
            Some((scattered, self.albedo.clone()))
        } else {
            None
        }
    }
}

impl Scatter<f64> for Materials<f64> {
    fn scatter(
        &self,
        ray_in: &Ray<f64>,
        hit_record: &HitRecord<f64>,
    ) -> Option<(Ray<f64>, Color<f64>)> {
        match self {
            Materials::Lambertian(lam) => lam.scatter(ray_in, hit_record),
            Materials::Metal(metal) => metal.scatter(ray_in, hit_record),
        }
    }
}
