use num_traits::Float;
use rand::distributions::uniform::SampleUniform;
use std::ops;
use std::slice::{Iter, IterMut};

use crate::base::vec3::Vec3;
use crate::base::Base3;
use crate::base::XYZ;
use raytracer_derive::Base3Ops;

#[derive(Debug, Default, Base3Ops, Clone)]
pub struct Point3<T>(pub Base3<T>);

impl<T: ops::Add<T, Output = T> + Copy> ops::Add<&Vec3<T>> for &Point3<T> {
    type Output = Point3<T>;

    fn add(self, other: &Vec3<T>) -> Self::Output {
        [self[0] + other[0], self[1] + other[1], self[2] + other[2]].into()
    }
}

impl<T: ops::Add<T, Output = T> + Copy> ops::Add<Vec3<T>> for Point3<T> {
    type Output = Point3<T>;

    fn add(self, other: Vec3<T>) -> Self::Output {
        [self[0] + other[0], self[1] + other[1], self[2] + other[2]].into()
    }
}

impl<T: ops::Sub<T, Output = T> + Copy> ops::Sub<&Vec3<T>> for &Point3<T> {
    type Output = Point3<T>;

    fn sub(self, other: &Vec3<T>) -> Self::Output {
        [self[0] - other[0], self[1] - other[1], self[2] - other[2]].into()
    }
}

impl<T: ops::Sub<T, Output = T> + Copy> ops::Sub<Vec3<T>> for Point3<T> {
    type Output = Point3<T>;

    fn sub(self, other: Vec3<T>) -> Self::Output {
        [self[0] - other[0], self[1] - other[1], self[2] - other[2]].into()
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

impl<T: Float> Point3<T> {
    pub fn vec_from(&self, origin: &Point3<T>) -> Vec3<T> {
        Vec3((self - origin).0)
    }
}
