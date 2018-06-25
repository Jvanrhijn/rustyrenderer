use std::vec;
use std::mem;
use std::convert;
extern crate num;
use geo;
use image;

pub trait Polygon<T>
{
    fn draw(&self, img: &mut image::RgbImage, color: &[u8; 3]);

    fn draw_filled(&self, img: &mut image::RgbImage, color: &[u8; 3]);

    fn vertices(&self) -> vec::Vec<&geo::Vec3<T>>;
}

pub struct Line<T> {
    start: geo::Vec3<T>,
    end: geo::Vec3<T>,
}

impl<T> Line<T> {

    pub fn new(start: geo::Vec3<T>, end: geo::Vec3<T>) -> Line<T> {
        Line{start, end}
    }

}

impl<T> Polygon<T> for Line<T>
    where T: geo::Number<T> + num::ToPrimitive
{

    fn draw(&self, img: &mut image::RgbImage, color: &[u8; 3]) {
        // Inefficient implementation of Bresenham
        let Line{start, end} = self;
        let (mut x0, mut y0) = (start.x.to_u32().unwrap(), start.y.to_u32().unwrap());
        let (mut x1, mut y1) = (end.x.to_u32().unwrap(), end.y.to_u32().unwrap());
        let steep = (x1 as i32 - x0 as i32).abs() < (y1 as i32 - y0 as i32).abs();
        if steep {
            mem::swap(&mut x0, &mut y0);
            mem::swap(&mut y1, &mut x1);
        }
        if x0 > x1 {
            mem::swap(&mut x0, &mut x1);
            mem::swap(&mut y0, &mut y1);
        }
        for x in x0..=x1 {
            let t: f64 = (x - x0) as f64 / ((x1 - x0) as f64);
            let y = ((y0 as f64)*(1. - t) + (y1 as f64)*t) as u32;
            if steep {
                img.put_pixel(y, x, image::Rgb::<u8>(*color));
            } else {
                img.put_pixel(x, y, image::Rgb::<u8>(*color));
            }
        }
    }

    fn draw_filled(&self, img: &mut image::RgbImage, color: &[u8; 3]) {

    }

    fn vertices(&self) -> vec::Vec<&geo::Vec3<T>> {
        vec![&self.start, &self.end]
    }

}

pub struct Triangle<T> {
    a: geo::Vec3<T>,
    b: geo::Vec3<T>,
    c: geo::Vec3<T>,
}

impl<T> Triangle<T> {

    pub fn new(a: geo::Vec3<T>, b: geo::Vec3<T>, c: geo::Vec3<T>) -> Triangle<T> {
        // TODO create edges
        Triangle{a, b, c}
    }

}

impl<T> Polygon<T> for Triangle<T>
    where T: geo::Number<T>,
{

    fn draw(&self, img: &mut image::RgbImage, color: &[u8; 3]) {

    }

    fn draw_filled(&self, img: &mut image::RgbImage, color: &[u8; 3]) {

    }

    fn vertices(&self) -> vec::Vec<&geo::Vec3<T>> {
        vec![&self.a, &self.b, &self.c]
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triangle_create() {
        let triangle = Triangle::new(geo::Vec3f::new(1.0, 1.0, 0.0),
                                     geo::Vec3f::new(1.0, 2.0, 0.0),
                                     geo::Vec3f::new(0.0, 2.0, 0.0));
    }

    #[test]
    fn line_create() {
        let line = Line::new(geo::Vec3i::new(1, 2, 3), geo::Vec3i::new(4, 5, 6));
    }

    #[test]
    fn triangle_vertices() {
        let triangle = Triangle::new(geo::Vec3f::new(1.0, 1.0, 0.0),
                                     geo::Vec3f::new(1.0, 2.0, 0.0),
                                     geo::Vec3f::new(0.0, 2.0, 0.0));
        let verts = triangle.vertices();
        assert_eq!(verts[0], &geo::Vec3f::new(1.0, 1.0, 0.0));
        assert_eq!(verts[1], &geo::Vec3f::new(1.0, 2.0, 0.0));
        assert_eq!(verts[2], &geo::Vec3f::new(0.0, 2.0, 0.0));
    }

    #[test]
    fn line_points() {
        let line = Line::new(geo::Vec3i::new(1, 2, 3), geo::Vec3i::new(4, 5, 6));
        let points = line.vertices();
        assert_eq!(points[0], &geo::Vec3i::new(1, 2, 3));
        assert_eq!(points[1], &geo::Vec3i::new(4, 5, 6));
    }

}