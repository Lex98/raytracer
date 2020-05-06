use rand::distributions::uniform::SampleUniform;
use rand::prelude::*;
use std::ops;
use std::slice::{Iter, IterMut};

pub mod color;
pub mod point3;
pub mod vec3;

#[derive(Debug, Default)]
pub struct Base3<T>(pub [T; 3]);

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

impl<T: SampleUniform + Copy> Base3<T> {
    pub fn random(min: T, max: T) -> Self {
        let mut rng = thread_rng();
        Base3([
            rng.gen_range(min, max),
            rng.gen_range(min, max),
            rng.gen_range(min, max),
        ])
    }
}

pub trait XYZ {
    type Item;

    fn x(self) -> Self::Item;
    fn y(self) -> Self::Item;
    fn z(self) -> Self::Item;
}
