use crate::base::point3::Point3;
use crate::base::vec3::{Length, Vec3, Vec3Operations};
use crate::materials::Materials;
use crate::ray::Ray;

#[derive(Debug, Clone)]
pub struct HitRecord<T> {
    pub point: Point3<T>,
    pub normal: Vec3<T>,
    pub material: Materials<T>,
    pub t: T,
    pub front_face: bool,
}

#[derive(Debug, Clone)]
pub struct Sphere<T> {
    pub center: Point3<T>,
    pub radius: T,
    pub material: Materials<T>,
}

#[derive(Debug, Clone)]
pub enum Hittable<T> {
    Sphere(Sphere<T>),
    HittableVec(HittableVec<T>),
}

#[derive(Debug, Default, Clone)]
pub struct HittableVec<T> {
    pub objects: Vec<Hittable<T>>,
}

pub trait Hit<T> {
    fn hit(self, ray: &Ray<T>, t_min: T, t_max: T) -> Option<HitRecord<T>>;
}

impl HitRecord<f64> {
    fn new(
        point: Point3<f64>,
        t: f64,
        ray: &Ray<f64>,
        outward_normal: Vec3<f64>,
        material: &Materials<f64>,
    ) -> Self {
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            point,
            normal,
            material: material.clone(),
            t,
            front_face,
        }
    }
}

impl Hit<f64> for &Sphere<f64> {
    fn hit(self, ray: &Ray<f64>, t_min: f64, t_max: f64) -> Option<HitRecord<f64>> {
        let oc = ray.origin.vec_from(&self.center);
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;
        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;
            let temp = if temp < t_min || temp > t_max {
                (-half_b + root) / a
            } else {
                temp
            };
            if temp < t_min || temp > t_max {
                return None;
            }
            let hit_point = ray.at(temp);
            let outward_normal = (&hit_point.vec_from(&self.center)) / self.radius;
            return Some(HitRecord::new(
                hit_point,
                temp,
                ray,
                outward_normal,
                &self.material,
            ));
        }
        None
    }
}

impl Hit<f64> for &Hittable<f64> {
    fn hit(self, ray: &Ray<f64>, t_min: f64, t_max: f64) -> Option<HitRecord<f64>> {
        match self {
            Hittable::Sphere(sphere) => sphere.hit(ray, t_min, t_max),
            Hittable::HittableVec(vec) => vec.hit(ray, t_min, t_max),
        }
    }
}

impl Hit<f64> for &HittableVec<f64> {
    fn hit(self, ray: &Ray<f64>, t_min: f64, t_max: f64) -> Option<HitRecord<f64>> {
        let mut temp_rec = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if let Some(rec) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = rec.t;
                temp_rec = Some(rec);
            }
        }
        temp_rec
    }
}

impl<T> HittableVec<T> {
    pub fn push(&mut self, value: Hittable<T>) {
        self.objects.push(value)
    }
}
