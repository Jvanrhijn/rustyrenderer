use std;
extern crate num;
use std::cmp;
use std::ops;
use std::vec;
use std::convert;
use std::fmt;

pub trait Number<T>: Copy + num::Num + num::Bounded + num::NumCast + PartialOrd + Clone {}
impl<T> Number<T> for T where T: Copy + num::Num + num::Bounded + num::NumCast + PartialOrd + Clone {}

pub trait Vector<'a, T>
    where &'a Self: ops::Mul<&'a Self, Output=T> + 'a,
                 T: Number<T>
{
    fn from_vec(vec: &vec::Vec<T>) -> Self;

    fn dot(&'a self, other: &'a Self) -> T {
        self*other
    }

    fn norm(&'a self) -> f64 {
        (self.dot(self).to_f64().unwrap()).sqrt()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T>
    where T: Number<T>
{
    pub fn new(x: T, y: T) -> Vec2<T> {
        Vec2 { x, y }
    }

    pub fn normalize(self) -> Vec2<f64> {
        let norm = self.norm();
        let Vec2::<T>{x, y} = self;
        Vec2::<f64>{x: x.to_f64().unwrap()/norm, y: y.to_f64().unwrap()/norm}
    }

}


impl<'a, 'b, T> ops::Add<&'b Vec2<T>> for Vec2<T>
    where T: Number<T>
{
    type Output = Vec2<T>;
    fn add(self, other: &'b Vec2<T>) -> Vec2<T> {
        Vec2::<T>{
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<'a, 'b, T> ops::Sub<&'b Vec2<T>> for Vec2<T>
    where T: Number<T>
{
    type Output = Vec2<T>;
    fn sub(self, other: &'b Vec2<T>) -> Vec2<T> {
        Vec2::<T>{
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<'a, T> ops::Mul<T> for &'a Vec2<T>
    where T: Number<T>
{
    type Output = Vec2<T>;
    fn mul(self, scal: T) -> Vec2<T> {
        Vec2::<T>{
            x: self.x*scal,
            y: self.y*scal,
        }
    }
}

impl<'a, 'b, T> ops::Mul<&'b Vec2<T>> for &'a Vec2<T>
    where T: Number<T>
{
    type Output = T;
    fn mul(self, other: &'b Vec2<T>) -> T {
        self.x*other.x + self.y*other.y
    }
}

impl<T: fmt::Display> fmt::Display for Vec2<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vec2(x: {}, y: {})", self.x, self.y)
    }
}

impl<'a, T: 'a> Vector<'a, T> for Vec2<T>
    where T: Number<T> + 'a
{

    fn from_vec(vec: &vec::Vec<T>) -> Vec2<T> {
        if vec.len() != 2 {
            panic!("Can only initialize Vec2 from std::Vec if std::Vec has 2 elements");
        }
        Vec2::new(vec[0].clone(), vec[1].clone())
    }

}

// Vec3 impl

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Vec3<T>
{
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T> Vec3<T>
    where T: Number<T>
{
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3{x, y, z}
    }

    pub fn from_vec(vec: &vec::Vec<T>) -> Vec3<T> {
        if vec.len() != 3 {
            panic!("Can only initialize Vec3 from std::Vec if std::Vec has 3 elements");
        }
        Vec3{x: vec[0].clone(), y: vec[1].clone(), z: vec[2].clone()}
    }

    pub fn normalize(self) -> Vec3<f64> {
        let norm = self.norm();
        let Vec3::<T>{x, y, z} = self;
        Vec3::<f64>{
            x: x.to_f64().unwrap()/norm,
            y: y.to_f64().unwrap()/norm,
            z: z.to_f64().unwrap()/norm
        }
    }

    pub fn cross(&self, other: &Vec3<T>) -> Vec3<T> {
        let x = self.y*other.z - self.z*other.y;
        let y = self.z*other.x - self.x*other.z;
        let z = self.x*other.y - self.y*other.x;
        Vec3::<T>{x, y, z}
    }

}

impl<'a, 'b, T> ops::Add<&'b Vec3<T>> for &'a Vec3<T>
    where T: Number<T>
{
    type Output = Vec3<T>;
    fn add(self, other: &'b Vec3<T>) ->Vec3<T> {
        Vec3::<T>{
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl<'a, 'b, T> ops::Sub<&'b Vec3<T>> for &'a Vec3<T>
    where T: Number<T>
{
    type Output = Vec3<T>;
    fn sub(self, other: &'b Vec3<T>) -> Vec3<T> {
        Vec3::<T>{
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl<'a, T> ops::Mul<T> for &'a Vec3<T>
    where T: Number<T>
{
    type Output = Vec3<T>;
    fn mul(self, scal: T) -> Vec3<T> {
        Vec3::<T>{
            x: self.x*scal,
            y: self.y*scal,
            z: self.z*scal
        }
    }
}

impl<'a, 'b, T> ops::Mul<&'b Vec3<T>> for &'a Vec3<T>
    where T: Number<T>
{
    type Output = T;
    fn mul(self, other: &'b Vec3<T>) -> T {
        self.x*other.x + self.y*other.y + self.z*other.z
    }
}

impl<T: fmt::Display> fmt::Display for Vec3<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vec3(x: {}, y: {}, z: {})", self.x, self.y, self.z)
    }
}

impl<'a, T: 'a> Vector<'a, T> for Vec3<T>
    where T: Number<T> + 'a
{

    fn from_vec(vec: &vec::Vec<T>) -> Vec3<T>
    {
        if vec.len() != 3 {
            panic!("Can only initialize Vec3 from std::Vec if std::Vec has 3 elements");
        }
        Vec3::<T>::new(vec[0].clone(), vec[1].clone(), vec[2].clone())
    }

}

// typedefs
pub type Vec3f = Vec3<f64>;
pub type Vec2f = Vec2<f64>;
pub type Vec2i = Vec2<i32>;
pub type Vec3i = Vec3<i32>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_vec() {
        assert_eq!(Vec3f{x: 1., y: 2., z: 3.}, Vec3f::from_vec(&vec![1., 2., 3.]));
    }

    #[test]
    fn add() {
        let first = Vec3f::new(1., 2., 3.);
        let second = Vec3f::new(2., 3., 4.);
        assert_eq!(&first + &second, Vec3f::new(3., 5., 7.));
    }

    #[test]
    fn sub() {
        let first = Vec3f::new(1., 2., 3.);
        let second = Vec3f::new(2., 3., 4.);
        assert_eq!(&first - &second, Vec3f::new(-1., -1., -1.));
    }

    #[test]
    fn norm() {
        assert_eq!(Vec3f::new(1., 1., 1.).norm(), (3. as f64).sqrt());
    }

    fn normalize() {
        assert_eq!(Vec3f::new(1., 1., 1.).normalize().norm(), 1 as f64);
    }

    #[test]
    fn dot() {
        let first = Vec2f::new(1., -1.);
        let second = Vec2f::new(1., 1.);
        assert_eq!(&first*&second, 0 as f64);
        assert_eq!(first.dot(&second), 0 as f64);
    }

    #[test]
    fn cross() {
        let first = Vec3f::new(1.0, 2.0, 3.0);
        let second = Vec3f::new(2.0, 3.0, 4.0);
        assert_eq!(first.cross(&second).dot(&first), 0 as f64);
        assert_eq!(first.cross(&second).dot(&second), 0 as f64);
        assert_eq!(second.cross(&first), (&first.cross(&second))*(-1 as f64));
    }
}

