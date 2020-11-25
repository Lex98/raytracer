use rand::prelude::*;

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
pub struct Dielectric<T> {
    pub ref_idx: T,
}

#[derive(Debug, Clone)]
pub enum Materials<T> {
    Lambertian(Lambertian<T>),
    Metal(Metal<T>),
    Dielectric(Dielectric<T>),
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
        let reflected = ray_in.direction.unit().reflect(&hit_record.normal);
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

impl Scatter<f64> for Dielectric<f64> {
    fn scatter(
        &self,
        ray_in: &Ray<f64>,
        hit_record: &HitRecord<f64>,
    ) -> Option<(Ray<f64>, Color<f64>)> {
        let etai_over_etat = if hit_record.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let unit_direction = ray_in.direction.unit();
        let cos_theta = (-&unit_direction).dot(&hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
        let mut rng = thread_rng();
        let reflect_prob = shlick(cos_theta, etai_over_etat);
        let new_direction =
            if (etai_over_etat * sin_theta > 1.0) | (reflect_prob > rng.gen_range(0.0, 1.0)) {
                unit_direction.reflect(&hit_record.normal)
            } else {
                unit_direction.refract(&hit_record.normal, etai_over_etat)
            };

        let scattered = Ray {
            origin: hit_record.point.clone(),
            direction: new_direction,
        };
        Some((scattered, Color([1.0, 1.0, 1.0].into())))
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
            Materials::Dielectric(dielectric) => dielectric.scatter(ray_in, hit_record),
        }
    }
}

fn shlick(cosine: f64, rex_idx: f64) -> f64 {
    let r0 = ((1.0 - rex_idx) / (1.0 + rex_idx)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
