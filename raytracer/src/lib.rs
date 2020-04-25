use raytracer_derive::Base3Ops;
use std::clone::Clone;
use std::io;
use std::iter::IntoIterator;
use std::ops;
use std::slice::{Iter, IterMut};

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Base3([f64; 3]);

#[derive(Debug, PartialEq, Default, Base3Ops, Clone)]
pub struct Color(pub Base3);

#[derive(Debug, PartialEq, Default, Base3Ops, Clone)]
pub struct Point3(pub Base3);

#[derive(Debug, PartialEq, Default, Base3Ops, Clone)]
pub struct Vec3(pub Base3);

impl From<[f64; 3]> for Base3 {
    fn from(base: [f64; 3]) -> Base3 {
        Base3(base)
    }
}

impl ops::Add for &Base3 {
    type Output = Base3;

    fn add(self, other: &Base3) -> Self::Output {
        Base3([
            self.0[0] + other.0[0],
            self.0[1] + other.0[1],
            self.0[2] + other.0[2],
        ])
    }
}

impl<'a> ops::AddAssign<&'a Base3> for Base3 {
    fn add_assign(&mut self, other: &'a Base3) {
        self.0[0] += other.0[0];
        self.0[1] += other.0[1];
        self.0[2] += other.0[2];
    }
}

impl ops::Div<f64> for &Base3 {
    type Output = Base3;

    fn div(self, divider: f64) -> Self::Output {
        Base3([
            self.0[0] / divider,
            self.0[1] / divider,
            self.0[2] / divider,
        ])
    }
}

impl ops::DivAssign<f64> for Base3 {
    fn div_assign(&mut self, divider: f64) {
        self.0[0] /= divider;
        self.0[1] /= divider;
        self.0[2] /= divider;
    }
}

impl ops::Index<usize> for Base3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl ops::IndexMut<usize> for Base3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl ops::Mul<f64> for &Base3 {
    type Output = Base3;

    fn mul(self, multiplier: f64) -> Self::Output {
        Base3([
            self.0[0] * multiplier,
            self.0[1] * multiplier,
            self.0[2] * multiplier,
        ])
    }
}

impl ops::MulAssign<f64> for Base3 {
    fn mul_assign(&mut self, multiplier: f64) {
        self.0[0] *= multiplier;
        self.0[1] *= multiplier;
        self.0[2] *= multiplier;
    }
}

impl<'a> ops::Neg for &'a Base3 {
    type Output = Base3;

    fn neg(self) -> Self::Output {
        Base3([-self.0[0], -self.0[1], -self.0[2]])
    }
}

impl ops::Sub for &Base3 {
    type Output = Base3;

    fn sub(self, other: &Base3) -> Self::Output {
        Base3([
            self.0[0] - other.0[0],
            self.0[1] - other.0[1],
            self.0[2] - other.0[2],
        ])
    }
}

impl<'a> ops::SubAssign<&'a Base3> for Base3 {
    fn sub_assign(&mut self, other: &'a Base3) {
        self.0[0] -= other.0[0];
        self.0[1] -= other.0[1];
        self.0[2] -= other.0[2];
    }
}

impl<'a> IntoIterator for &'a Base3 {
    type Item = &'a f64;
    type IntoIter = Iter<'a, f64>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut Base3 {
    type Item = &'a mut f64;
    type IntoIter = IterMut<'a, f64>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl<'a> Base3 {
    pub fn iter(&'a self) -> Iter<'a, f64> {
        self.into_iter()
    }

    pub fn iter_mut(&'a mut self) -> IterMut<'a, f64> {
        self.into_iter()
    }
}

pub trait Length {
    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    fn length_squared(&self) -> f64;
}

impl Length for Vec3 {
    fn length_squared(&self) -> f64 {
        self.iter().map(|x| x.powi(2)).sum()
    }
}

pub trait Write {
    fn write<T: io::Write>(&self, writer: &mut T) -> io::Result<()>;
}

impl Write for Color {
    fn write<T: std::io::Write>(&self, writer: &mut T) -> io::Result<()> {
        writer.write_fmt(format_args!(
            "{} {} {}\n",
            (self[0] * 255.999) as i32,
            (self[1] * 255.999) as i32,
            (self[2] * 255.999) as i32,
        ))
    }
}

pub trait Vec3Operations {
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

    fn unit_vector<'a>(&'a self) -> <&'a Self as std::ops::Div<f64>>::Output
    where
        Self: Length,
        &'a Self: ops::Div<f64>,
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

impl<'a> XYZ for &'a Vec3 {
    type Item = &'a f64;

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

impl Vec3Operations for Vec3 {}
