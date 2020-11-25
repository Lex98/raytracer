use num_traits::Float;
use rand::distributions::uniform::SampleUniform;
use rand::prelude::*;
use std::f64::consts::PI;
use std::ops;
use std::ops::{Add, Div, Mul, Sub};
use std::slice::{Iter, IterMut};

use crate::base::color::{AsColor, Color};
use crate::base::Base3;
use crate::base::XYZ;
use raytracer_derive::Base3Ops;

#[derive(Debug, Default, Base3Ops, Clone)]
pub struct Vec3<T>(pub Base3<T>);

pub trait Vec3Operations<T> {
    fn dot<'a>(
        &'a self,
        other: &'a Self,
    ) -> <<<<&'a Self as XYZ>::Item as Mul<<&'a Self as XYZ>::Item>>::Output as Add<
        <<&Self as XYZ>::Item as Mul<<&'a Self as XYZ>::Item>>::Output,
    >>::Output as Add<<<&'a Self as XYZ>::Item as Mul>::Output>>::Output
    where
        &'a Self: XYZ,
        <&'a Self as XYZ>::Item: Mul<<&'a Self as XYZ>::Item>,
        <<&'a Self as XYZ>::Item as Mul<<&'a Self as XYZ>::Item>>::Output:
            Add<<<&'a Self as XYZ>::Item as Mul<<&'a Self as XYZ>::Item>>::Output>,
        <<<&'a Self as XYZ>::Item as Mul<<&'a Self as XYZ>::Item>>::Output as Add<
            <<&'a Self as XYZ>::Item as Mul>::Output,
        >>::Output: Add<<<&'a Self as XYZ>::Item as Mul<<&'a Self as XYZ>::Item>>::Output>,
    {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    fn cross<'a>(&'a self, other: &'a Self) -> Self
    where
        &'a Self: XYZ,
        Self: From<
            [<<<&'a Self as XYZ>::Item as Mul<<&'a Self as XYZ>::Item>>::Output as Sub<
                <<&'a Self as XYZ>::Item as Mul>::Output,
            >>::Output; 3],
        >,
        <&'a Self as XYZ>::Item: Mul<<&'a Self as XYZ>::Item>,
        <<&'a Self as XYZ>::Item as Mul<<&'a Self as XYZ>::Item>>::Output:
            Sub<<<&'a Self as XYZ>::Item as Mul<<&'a Self as XYZ>::Item>>::Output>,
    {
        [
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        ]
        .into()
    }

    fn unit<'a>(&'a self) -> <&'a Self as Div<T>>::Output
    where
        T: Float,
        Self: Length<T>,
        &'a Self: Div<T>,
    {
        self / self.length()
    }
}

impl<T: Float> Vec3Operations<T> for Vec3<T> {}

impl Vec3<f64> {
    pub fn random_unit() -> Vec3<f64> {
        let mut rng = thread_rng();
        let a = rng.gen_range(0.0, 2.0 * PI);
        let z = rng.gen_range(-1.0, 1.0);
        let r = (1.0 - z.powi(2)).sqrt();
        Vec3([r * a.cos(), r * a.sin(), z].into())
    }

    pub fn reflect(&self, normal: &Vec3<f64>) -> Vec3<f64> {
        self - &(normal * self.dot(normal) * 2.0)
    }

    pub fn refract(&self, normal: &Vec3<f64>, etai_over_etat: f64) -> Vec3<f64> {
        let cos_theta = (-self).dot(normal);
        let r_out_parallel = (self + &(normal * cos_theta)) * etai_over_etat;
        let r_out_perp = -normal * (1.0 - r_out_parallel.length_squared()).sqrt();
        r_out_parallel + r_out_perp
    }
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

impl<T> AsColor<T> for Vec3<T> {
    fn as_color(self) -> Color<T> {
        Color(self.0)
    }
}
