use std::ops;
use std::convert;
use std::fmt;

pub trait Number<T>: Copy + ops::Add<Output=T> + ops::Mul<Output=T> + ops::Sub<Output=T> + ops::Div<Output=T>{}
impl<T> Number<T> for T where T: Copy + ops::Add<Output=T> + ops::Mul<Output=T> + ops::Sub<Output=T> + ops::Div<Output=T> {}

pub struct Vec2<T> {
    x: T,
    y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Vec2<T> {
        Vec2{x, y}
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

// Vec3 impl

pub struct Vec3<T>
{
    x: T,
    y: T,
    z: T
}

impl<T> Vec3<T>
    where T: Number<T> + convert::Into<f64>
{
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3{x, y, z}
    }

    pub fn norm(&self) -> f64 {
        (self.x*self.x + self.y*self.y + self.z*self.z).into().sqrt()
    }

    pub fn normalize(self) -> Vec3<f64> {
        let norm = self.norm();
        Vec3::<f64>{x: self.x.into()/norm, y: self.y.into()/norm, z: self.z.into()/norm}
    }

    pub fn cross(&self, other: &Vec3<T>) -> Vec3<T> {
        let x = self.y*other.z - self.z*other.y;
        let y = self.z*other.x - self.x*other.y;
        let z = self.x*other.y - self.y*other.z;
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

// typedefs
pub type Vec3f = Vec3<f64>;
pub type Vec2f = Vec2<f64>;
