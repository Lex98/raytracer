use std::ops;
use std::io;
use std::clone::Clone;
use raytracer_derive::Base3Ops;

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Base3([f64; 3]);

impl From<[f64; 3]> for Base3 {
    fn from(base :[f64; 3]) -> Base3 {
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
        Base3([
            -self.0[0],
            -self.0[1],
            -self.0[2],
        ])
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

#[derive(Debug, PartialEq, Default, Base3Ops, Clone)]
pub struct Color(pub Base3);

pub trait Length {
    fn length(&self) -> f64{
        self.length_squared().sqrt()
    }
    fn length_squared(&self) -> f64;
}

// impl Length for Base3 {
//     fn length_squared(&self) -> f64 {
//         self.0[0]*self.0[0] + self.0[1]*self.0[1] + self.0[2]*self.0[2]
//     }
// }

pub trait Write {
    fn write<T: io::Write>(&self, writer: &mut T) -> io::Result<()>;
}

impl Write for Color{
    fn write<T: std::io::Write>(&self, writer: &mut T) -> io::Result<()>{
        writer.write_fmt(format_args!(
            "{} {} {}\n",
            (self[0] * 255.999) as i32,
            (self[1] * 255.999) as i32,
            (self[2] * 255.999) as i32,
        ))
    }
}

pub trait VecOperations {
    fn dot(&self, other: Self) -> Self;
    fn cross(&self, other: Self) -> Self;
    fn unit_vector<'a>(&'a self) -> <&'a Self as std::ops::Div<f64>>::Output
    where Self: std::marker::Sized,
    &'a Self: ops::Div<f64> + Length {
        self / self.length()
    }
}
