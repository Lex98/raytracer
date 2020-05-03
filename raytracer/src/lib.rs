use image::Rgb;
use num_traits::Float;
use raytracer_derive::Base3Ops;
use std::f64::INFINITY;
use std::iter::IntoIterator;
use std::ops;
use std::ops::{Add, Mul, Sub, Div};
use std::slice::{Iter, IterMut};

#[derive(Debug, Default)]
pub struct Base3<T>(pub [T; 3]);

#[derive(Debug, Default, Base3Ops, Clone)]
pub struct Color<T>(pub Base3<T>);

#[derive(Debug, Default, Base3Ops, Clone)]
pub struct Point3<T>(pub Base3<T>);

#[derive(Debug, Default, Base3Ops, Clone)]
pub struct Vec3<T>(pub Base3<T>);

#[derive(Debug, Default, Clone)]
pub struct Ray<T> {
    pub origin: Point3<T>,
    pub direction: Vec3<T>,
}

#[derive(Debug, Default, Clone)]
pub struct HitRecord<T> {
    pub point: Point3<T>,
    pub normal: Vec3<T>,
    pub t: T,
    pub front_face: bool,
}

#[derive(Debug, Default, Clone)]
pub struct Sphere<T> {
    pub center: Point3<T>,
    pub radius: T,
}

#[derive(Debug, Clone)]
pub enum Hittable<T> {
    Sphere(Sphere<T>),
}

#[derive(Debug, Default, Clone)]
pub struct HittableVec<T> {
    pub objects: Vec<Hittable<T>>,
}

#[derive(Debug, Clone)]
pub struct Camera<T> {
    pub upper_left_corner: Point3<T>,
    pub horizontal: Vec3<T>,
    pub vertical: Vec3<T>,
    pub origin : Point3<T>,
}

impl<T> From<[T; 3]> for Base3<T> {
    fn from(base: [T; 3]) -> Base3<T> {
        Base3(base)
    }
}

impl<T: Clone> Clone for Base3<T> {
    fn clone(&self) -> Self {
        Base3(self.0.clone())
    }
}

impl<T: ops::Add<T, Output = T> + Copy> ops::Add for &Base3<T> {
    type Output = Base3<T>;

    fn add(self, other: &Base3<T>) -> Self::Output {
        [
            self.0[0] + other.0[0],
            self.0[1] + other.0[1],
            self.0[2] + other.0[2],
        ]
        .into()
    }
}

impl<T: ops::Add<T, Output = T> + Copy> ops::Add for Base3<T> {
    type Output = Base3<T>;

    fn add(self, other: Base3<T>) -> Self::Output {
        [
            self.0[0] + other.0[0],
            self.0[1] + other.0[1],
            self.0[2] + other.0[2],
        ]
        .into()
    }
}

impl<'a, T: ops::AddAssign<T> + Copy> ops::AddAssign<&'a Base3<T>> for Base3<T> {
    fn add_assign(&mut self, other: &'a Base3<T>) {
        self.0[0] += other.0[0];
        self.0[1] += other.0[1];
        self.0[2] += other.0[2];
    }
}

impl<T: ops::Div<T, Output = T> + Copy> ops::Div<T> for &Base3<T> {
    type Output = Base3<T>;

    fn div(self, divider: T) -> Self::Output {
        [
            self.0[0] / divider,
            self.0[1] / divider,
            self.0[2] / divider,
        ]
        .into()
    }
}

impl<T: ops::Div<T, Output = T> + Copy> ops::Div<T> for Base3<T> {
    type Output = Base3<T>;

    fn div(self, divider: T) -> Self::Output {
        [
            self.0[0] / divider,
            self.0[1] / divider,
            self.0[2] / divider,
        ]
        .into()
    }
}

impl<T: ops::DivAssign<T> + Copy> ops::DivAssign<T> for Base3<T> {
    fn div_assign(&mut self, divider: T) {
        self.0[0] /= divider;
        self.0[1] /= divider;
        self.0[2] /= divider;
    }
}

impl<T> ops::Index<usize> for Base3<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> ops::IndexMut<usize> for Base3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: ops::Mul<T, Output = T> + Copy> ops::Mul<T> for &Base3<T> {
    type Output = Base3<T>;

    fn mul(self, multiplier: T) -> Self::Output {
        [
            self.0[0] * multiplier,
            self.0[1] * multiplier,
            self.0[2] * multiplier,
        ]
        .into()
    }
}

impl<T: ops::Mul<T, Output = T> + Copy> ops::Mul<T> for Base3<T> {
    type Output = Base3<T>;

    fn mul(self, multiplier: T) -> Self::Output {
        [
            self.0[0] * multiplier,
            self.0[1] * multiplier,
            self.0[2] * multiplier,
        ]
        .into()
    }
}

impl<T: ops::MulAssign<T> + Copy> ops::MulAssign<T> for Base3<T> {
    fn mul_assign(&mut self, multiplier: T) {
        self.0[0] *= multiplier;
        self.0[1] *= multiplier;
        self.0[2] *= multiplier;
    }
}

impl<'a, T: ops::Neg<Output = T> + Copy> ops::Neg for &'a Base3<T> {
    type Output = Base3<T>;

    fn neg(self) -> Self::Output {
        [-self.0[0], -self.0[1], -self.0[2]].into()
    }
}

impl<T: ops::Neg<Output = T> + Copy> ops::Neg for Base3<T> {
    type Output = Base3<T>;

    fn neg(self) -> Self::Output {
        [-self.0[0], -self.0[1], -self.0[2]].into()
    }
}

impl<T: ops::Sub<T, Output = T> + Copy> ops::Sub for &Base3<T> {
    type Output = Base3<T>;

    fn sub(self, other: &Base3<T>) -> Self::Output {
        [
            self.0[0] - other.0[0],
            self.0[1] - other.0[1],
            self.0[2] - other.0[2],
        ]
        .into()
    }
}

impl<T: ops::Sub<T, Output = T> + Copy> ops::Sub for Base3<T> {
    type Output = Base3<T>;

    fn sub(self, other: Base3<T>) -> Self::Output {
        [
            self.0[0] - other.0[0],
            self.0[1] - other.0[1],
            self.0[2] - other.0[2],
        ]
        .into()
    }
}

impl<'a, T: ops::SubAssign<T> + Copy> ops::SubAssign<&'a Base3<T>> for Base3<T> {
    fn sub_assign(&mut self, other: &'a Base3<T>) {
        self.0[0] -= other.0[0];
        self.0[1] -= other.0[1];
        self.0[2] -= other.0[2];
    }
}

impl<'a, T> IntoIterator for &'a Base3<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Base3<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl<'a, T> Base3<T> {
    pub fn iter(&'a self) -> Iter<'a, T> {
        self.into_iter()
    }

    pub fn iter_mut(&'a mut self) -> IterMut<'a, T> {
        self.into_iter()
    }
}

pub trait Length<T: Float> {
    fn length(&self) -> T {
        self.length_squared().sqrt()
    }
    fn length_squared(&self) -> T;
}

impl<T: std::iter::Sum + Float> Length<T> for Vec3<T> {
    fn length_squared(&self) -> T {
        self.iter().map(|x| x.powi(2)).sum()
    }
}

pub trait Vec3Operations<T> {
    fn dot<'a, 'b: 'a>(&'a self, other: &'b Self) -> <<<<&'a Self as XYZ>::Item as Mul<<&'a Self as XYZ>::Item>>::Output as Add<<<&Self as XYZ>::Item as Mul<<&'a Self as XYZ>::Item>>::Output>>::Output as Add<<<&'a Self as XYZ>::Item as Mul>::Output>>::Output
    where &'a Self: XYZ,
    <&'a Self as XYZ>::Item: Mul<<&'a Self as XYZ>::Item>,
    <<&'a Self as XYZ>::Item as Mul<<&'a Self as XYZ>::Item>>::Output: Add<<<&'a Self as XYZ>::Item as Mul<<&'a Self as XYZ>::Item>>::Output>,
    <<<&'a Self as XYZ>::Item as Mul<<&'a Self as XYZ>::Item>>::Output as Add<<<&'a Self as XYZ>::Item as Mul>::Output>>::Output: Add<<<&'a Self as XYZ>::Item as Mul<<&'a Self as XYZ>::Item>>::Output>
    {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    fn cross<'a, 'b: 'a>(&'a self, other: &'b Self) -> Self
    where &'a Self: XYZ,
    Self: From<[<<<&'a Self as XYZ>::Item as Mul<<&'a Self as XYZ>::Item>>::Output as Sub<<<&'a Self as XYZ>::Item as Mul>::Output>>::Output; 3]>,
    <&'a Self as XYZ>::Item: Mul<<&'a Self as XYZ>::Item>,
    <<&'a Self as XYZ>::Item as Mul<<&'a Self as XYZ>::Item>>::Output: Sub<<<&'a Self as XYZ>::Item as Mul<<&'a Self as XYZ>::Item>>::Output>
    {
        [
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        ]
        .into()
    }

    fn unit_vector<'a>(&'a self) -> <&'a Self as Div<T>>::Output
    where
        T: Float,
        Self: Length<T>,
        &'a Self: Div<T>,
    {
        self / self.length()
    }
}

pub trait XYZ {
    type Item;

    fn x(self) -> Self::Item;
    fn y(self) -> Self::Item;
    fn z(self) -> Self::Item;
}

impl<'a, T> XYZ for &'a Vec3<T> {
    type Item = &'a T;

    fn x(self) -> Self::Item {
        &self[0]
    }

    fn y(self) -> Self::Item {
        &self[1]
    }

    fn z(self) -> Self::Item {
        &self[2]
    }
}

impl<'a, T> XYZ for &'a Point3<T> {
    type Item = &'a T;

    fn x(self) -> Self::Item {
        &self[0]
    }

    fn y(self) -> Self::Item {
        &self[1]
    }

    fn z(self) -> Self::Item {
        &self[2]
    }
}

pub trait RGB {
    type Item;

    fn r(self) -> Self::Item;
    fn g(self) -> Self::Item;
    fn b(self) -> Self::Item;
}

impl<'a, T> RGB for &'a Color<T> {
    type Item = &'a T;

    fn r(self) -> Self::Item {
        &self[0]
    }

    fn g(self) -> Self::Item {
        &self[1]
    }

    fn b(self) -> Self::Item {
        &self[2]
    }
}

impl<T: Float> Vec3Operations<T> for Vec3<T> {}

impl<T: ops::Add<T, Output = T> + Copy> ops::Add<&Vec3<T>> for &Point3<T> {
    type Output = Point3<T>;

    fn add(self, other: &Vec3<T>) -> Self::Output {
        [self[0] + other[0], self[1] + other[1], self[2] + other[2]].into()
    }
}

pub trait RayTrait<T> {
    fn at(self, t: T) -> Point3<T>;
    fn color(self) -> Color<T>;
}

impl RayTrait<f64> for &Ray<f64> {
    fn at(self, t: f64) -> Point3<f64> {
        &self.origin + &(&self.direction * t)
    }

    fn color(self) -> Color<f64> {
        let t = 0.5 * (self.direction.unit_vector().y() + 1.0);
        Color([1., 1., 1.].into()) * (1.0 - t) + Color([0.5, 0.7, 1.].into()) * t
    }
}

impl<T: Into<f64> + Copy> From<Color<T>> for Rgb<u8> {
    fn from(color: Color<T>) -> Rgb<u8> {
        Rgb([
            (color[0].into() * 255.999) as u8,
            (color[1].into() * 255.999) as u8,
            (color[2].into() * 255.999) as u8,
        ])
    }
}

impl<T: Float + Into<f64>> Color<T> {
    pub fn as_rgb(&self, samples_per_pixel: i32) -> Rgb<u8> {
        let scale = 1.0 / samples_per_pixel as f64;
        let r = scale * (*self.r()).into();
        let g = scale * (*self.g()).into();
        let b = scale * (*self.b()).into();
        Rgb([
            (clamp(r, 0.0, 0.999) * 256.0) as u8,
            (clamp(g, 0.0, 0.999) * 256.0) as u8,
            (clamp(b, 0.0, 0.999) * 256.0) as u8,
        ])
    }
}

pub fn ray_color<T: Hit<f64>>(ray: &Ray<f64>, world: T) -> Color<f64> {
    match world.hit(&ray, 0.0, INFINITY) {
        Some(rec) => (rec.normal + Vec3([1.0, 1.0, 1.0].into())).as_color() * 0.5,
        None => {
            let t = 0.5 * (ray.direction.unit_vector().y() + 1.0);
            Color([1.0, 1.0, 1.0].into()) * (1.0 - t) + Color([0.5, 0.7, 1.0].into()) * t
        }
    }
}

impl<T: Float> Point3<T> {
    pub fn vec_from(&self, origin: &Point3<T>) -> Vec3<T> {
        Vec3((self - origin).0)
    }
}

pub trait Hit<T> {
    fn hit(self, ray: &Ray<T>, t_min: T, t_max: T) -> Option<HitRecord<T>>;
}

impl HitRecord<f64> {
    fn new(point: Point3<f64>, t: f64, ray: &Ray<f64>, outward_normal: Vec3<f64>) -> Self {
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            point,
            normal,
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
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;
            let temp = if temp < t_min || temp > t_max {
                (-half_b + root) / a
            } else {
                temp
            };
            if temp < t_min || temp > t_max {
                return None
            }
            let hit_point = ray.at(temp);
            let outward_normal = (&hit_point.vec_from(&self.center)) / self.radius;
            return Some(HitRecord::new(hit_point, temp, ray, outward_normal));
        }
        None
    }
}

impl Hit<f64> for &Hittable<f64> {
    fn hit(self, ray: &Ray<f64>, t_min: f64, t_max: f64) -> Option<HitRecord<f64>> {
        match self {
            Hittable::Sphere(sphere) => sphere.hit(ray, t_min, t_max),
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

pub trait AsColor<T> {
    fn as_color(self) -> Color<T>;
}

impl<T> AsColor<T> for Vec3<T> {
    fn as_color(self) -> Color<T> {
        Color(self.0)
    }
}

impl<T> HittableVec<T> {
    pub fn push(&mut self, value: Hittable<T>) {
        self.objects.push(value)
    }
}

impl Default for Camera<f64> {
    fn default() -> Self {
        Camera {
            upper_left_corner: Point3([-2.0, 1.0, 1.0].into()),
            horizontal: Vec3([4.0, 0.0, 0.0].into()),
            vertical: Vec3([0.0, -2.0, 0.0].into()),
            origin : Point3::default()
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

pub fn clamp<T: PartialOrd>(x: T, min: T, max: T) -> T {
    if x < min {return min};
    if x > max {return max};
    x
}
