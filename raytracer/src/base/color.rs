use image::Rgb;
use num_traits::Float;
use rand::distributions::uniform::SampleUniform;
use std::ops;
use std::slice::{Iter, IterMut};

use crate::base::Base3;
use crate::utils::clamp;
use raytracer_derive::Base3Ops;

#[derive(Debug, Default, Base3Ops, Clone)]
pub struct Color<T>(pub Base3<T>);

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

impl<T: Float + Into<f64>> Color<T> {
    pub fn as_rgb(&self, samples_per_pixel: u32) -> Rgb<u8> {
        let scale = 1.0 / samples_per_pixel as f64;
        let r = (scale * (*self.r()).into()).sqrt();
        let g = (scale * (*self.g()).into()).sqrt();
        let b = (scale * (*self.b()).into()).sqrt();
        Rgb([
            (clamp(r, 0.0, 0.999) * 256.0) as u8,
            (clamp(g, 0.0, 0.999) * 256.0) as u8,
            (clamp(b, 0.0, 0.999) * 256.0) as u8,
        ])
    }
}

pub trait AsColor<T> {
    fn as_color(self) -> Color<T>;
}

impl<T: ops::Mul<T, Output = T> + Copy> ops::Mul<Color<T>> for Color<T> {
    type Output = Color<T>;

    fn mul(self, other: Color<T>) -> Self::Output {
        [self[0] * other[0], self[1] * other[1], self[2] * other[2]].into()
    }
}

impl<T: ops::Mul<T, Output = T> + Copy> ops::Mul<&Color<T>> for &Color<T> {
    type Output = Color<T>;

    fn mul(self, other: &Color<T>) -> Self::Output {
        [self[0] * other[0], self[1] * other[1], self[2] * other[2]].into()
    }
}
