use image::Rgb;
use num_traits::Float;
use raytracer_derive::Base3Ops;
use std::io;
use std::iter::IntoIterator;
use std::ops;
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

pub trait Write {
    fn write<T: io::Write>(&self, writer: &mut T) -> io::Result<()>;
}

impl<W> Write for Color<W>
where
    W: Copy + Into<f64>,
{
    fn write<T: io::Write>(&self, writer: &mut T) -> io::Result<()> {
        writer.write_fmt(format_args!(
            "{} {} {}\n",
            (self[0].into() * 255.999) as i32,
            (self[1].into() * 255.999) as i32,
            (self[2].into() * 255.999) as i32,
        ))
    }
}

pub trait Vec3Operations<T> {
    fn dot<'a, 'b: 'a>(&'a self, other: &'b Self) -> <<<<&'a Self as XYZ>::Item as std::ops::Mul<<&'a Self as XYZ>::Item>>::Output as std::ops::Add<<<&Self as XYZ>::Item as std::ops::Mul<<&'a Self as XYZ>::Item>>::Output>>::Output as std::ops::Add<<<&'a Self as XYZ>::Item as std::ops::Mul>::Output>>::Output
    where &'a Self: XYZ,
    <&'a Self as XYZ>::Item: ops::Mul<<&'a Self as XYZ>::Item>,
    <<&'a Self as XYZ>::Item as std::ops::Mul<<&'a Self as XYZ>::Item>>::Output: ops::Add<<<&'a Self as XYZ>::Item as std::ops::Mul<<&'a Self as XYZ>::Item>>::Output>,
    <<<&'a Self as XYZ>::Item as std::ops::Mul<<&'a Self as XYZ>::Item>>::Output as std::ops::Add<<<&'a Self as XYZ>::Item as std::ops::Mul>::Output>>::Output: ops::Add<<<&'a Self as XYZ>::Item as std::ops::Mul<<&'a Self as XYZ>::Item>>::Output>
    {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    fn cross<'a, 'b: 'a>(&'a self, other: &'b Self) -> Self
    where
    Self: From<[<<<&'a Self as XYZ>::Item as std::ops::Mul<<&'a Self as XYZ>::Item>>::Output as std::ops::Sub<<<&'a Self as XYZ>::Item as std::ops::Mul>::Output>>::Output; 3]>,
    &'a Self: XYZ,
    <&'a Self as XYZ>::Item: ops::Mul<<&'a Self as XYZ>::Item>,
    <<&'a Self as XYZ>::Item as std::ops::Mul<<&'a Self as XYZ>::Item>>::Output: ops::Sub<<<&'a Self as XYZ>::Item as std::ops::Mul<<&'a Self as XYZ>::Item>>::Output>
    {
        [
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        ]
        .into()
    }

    fn unit_vector<'a>(&'a self) -> <&'a Self as std::ops::Div<T>>::Output
    where
        T: Float,
        Self: Length<T>,
        &'a Self: ops::Div<T>,
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

pub fn ray_color(ray: &Ray<f64>) -> Color<f64> {
    let t = hit_sphere(&Point3([0.0, 0.0, 1.0].into()), 0.5, ray);
    match t {
        Some(t) => {
            let n = (ray.at(t) - Point3([0.0, 0.0, 1.0].into()))
                .vec_from(&Point3::default())
                .unit_vector();
            Color([n.x() + 1.0, n.y() + 1.0, n.z() + 1.0].into()) * 0.5
        }
        None => {
            let t = 0.5 * (ray.direction.unit_vector().y() + 1.0);
            Color([0.5, 0.7, 1.0].into()) * (1.0 - t) + Color([1.0, 1.0, 1.0].into()) * t
        }
    }
}

impl<T: Float> Point3<T> {
    pub fn vec_from(&self, origin: &Point3<T>) -> Vec3<T> {
        Vec3(((self - origin).0).0.into())
    }
}

pub fn hit_sphere(center: &Point3<f64>, radius: f64, ray: &Ray<f64>) -> Option<f64> {
    let oc = ray.origin.vec_from(center);
    let a = ray.direction.length_squared();
    let half_b = oc.dot(&ray.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        None
    } else {
        Some((-half_b - discriminant.sqrt()) / a)
    }
}
