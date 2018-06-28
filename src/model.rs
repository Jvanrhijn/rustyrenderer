use std::cmp;
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

impl<T> Line<T>
    where T: geo::Number<T>
{

    pub fn new(start: geo::Vec3<T>, end: geo::Vec3<T>) -> Line<T> {
        let (start, end) = match start.y.partial_cmp(&end.y)
            .unwrap_or(cmp::Ordering::Equal) {
            cmp::Ordering::Greater => (start, end),
            cmp::Ordering::Less => (end, start),
            cmp::Ordering::Equal => (start, end)
        };
        Line{start, end}
    }

    pub fn fill_between(&self, other: &Line<T>, img: &mut image::RgbImage) {
        //rasterize
        let self_rast = self.rasterize(img);
        let other_rast = other.rasterize(img);
        let (highest, lowest) = match self_rast.start.y.partial_cmp(&other_rast.start.y)
            .unwrap_or(cmp::Ordering::Equal) {
            cmp::Ordering::Greater => (self_rast, other_rast),
            cmp::Ordering::Less    => (other_rast, self_rast),
            cmp::Ordering::Equal   => (self_rast, other_rast)
        };
    }

    fn rasterize(&self, img: &image::RgbImage) -> Line<u32> {
        let (imgx, imgy) = img.dimensions();
        let (imgx, imgy) = (imgx-1, imgy-1);
        let start = geo::Vec3::<u32>::new(((self.start.x.to_f64().unwrap() + 1.)*0.5*(imgx as f64)) as u32,
                                                   ((self.start.y.to_f64().unwrap() + 1.)*0.5*(imgy as f64)) as u32, 0);
        let end = geo::Vec3::<u32>::new(((self.start.x.to_f64().unwrap() + 1.)*0.5*(imgx as f64)) as u32,
                                                 ((self.start.y.to_f64().unwrap() + 1.)*0.5*(imgy as f64)) as u32, 0);
        Line::new(start, end)
    }

}

impl<T> Polygon<T> for Line<T>
    where T: geo::Number<T>
{

    fn draw(&self, img: &mut image::RgbImage, color: &[u8; 3]) {
        // most of this code should go into iterator
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
        let dx = (x1 as i32 - x0 as i32);
        let dy = (y1 as i32 - y0 as i32);
        let derror = dy.abs()*2;
        let mut error = 0;
        let mut y = y0 as i32;
        for x in x0..=x1 {
            // The actual 'put pixel' part should then put the pixel on the position as returned
            // by the iterator
            if steep {
                img.put_pixel(y as u32, x as u32, image::Rgb::<u8>(*color));
            } else {
                img.put_pixel(x as u32, y as u32, image::Rgb::<u8>(*color));
            }
            error += derror;
            if error > dx {
                y += if dy > 0 {1} else {-1};
                error -= dx*2;
            }
        }
    }

    fn draw_filled(&self, img: &mut image::RgbImage, color: &[u8; 3]) {

    }

    fn vertices(&self) -> vec::Vec<&geo::Vec3<T>> {
        vec![&self.start, &self.end]
    }

}

struct LineIterator<'a, T: 'a>
    where T: geo::Number<T>
{
    img: &'a image::RgbImage,
    line: &'a Line<T>,
    pixel: geo::Vec3<u32>,
}

impl<'a, T> LineIterator<'a, T>
    where T: geo::Number<T>
{
    pub fn new(line: &'a Line<T>, img: &'a image::RgbImage) -> LineIterator<'a, T> {
        let pixel = geo::Vec3::<u32>::new(line.start.x.to_u32().unwrap(),line.start.y.to_u32().unwrap(), 0);
        LineIterator{img, line, pixel}
    }
}

impl<'a, T> Iterator for LineIterator<'a, T>
    where T: geo::Number<T>
{
    type Item = geo::Vec3i;

    fn next(&mut self) -> Option<geo::Vec3i> {
        Some(geo::Vec3i::new(0, 0, 0))
    }
}

pub struct Triangle<T> {
    a: geo::Vec3<T>,
    b: geo::Vec3<T>,
    c: geo::Vec3<T>,
    edges: vec::Vec<Line<T>>,
}

impl<T> Triangle<T>
    where T: geo::Number<T>
{

    pub fn new(a: geo::Vec3<T>, b: geo::Vec3<T>, c: geo::Vec3<T>) -> Triangle<T> {
        let ab = Line::new(a.clone(), b.clone());
        let bc = Line::new(b.clone(), c.clone());
        let ac = Line::new(a.clone(), c.clone());
        Triangle{a, b, c, edges: vec![ab, bc, ac]}
    }

}

impl<T> Polygon<T> for Triangle<T>
    where T: geo::Number<T> + num::ToPrimitive
{

    fn draw(&self, img: &mut image::RgbImage, color: &[u8; 3]) {
        let (a, b, c) = match &self.edges.as_slice() {
            [first, second, third] => (first, second, third),
            _ => unreachable!()
        };
        a.draw(img, color);
        b.draw(img, color);
        c.draw(img, color);
    }

    fn draw_filled(&self, img: &mut image::RgbImage, color: &[u8; 3]) {
        // first order vertices by y-coordinate
        let vertices = self.vertices()
            .sort_by(|x, y| x.y.partial_cmp(&y.y).unwrap_or(cmp::Ordering::Equal));
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
        let a = geo::Vec3f::new(1.0, 1.0, 0.0);
        let b = geo::Vec3f::new(1.0, 2.0, 0.0);
        let c = geo::Vec3f::new(0.0, 2.0, 0.0);
        let triangle = Triangle::new(a, b, c);
    }

    #[test]
    fn line_create() {
        let a = geo::Vec3f::new(1.0, 1.0, 0.0);
        let b = geo::Vec3f::new(1.0, 2.0, 0.0);
        let c = geo::Vec3f::new(0.0, 2.0, 0.0);
        let triangle = Triangle::new(a, b, c);
    }

    #[test]
    fn triangle_vertices() {
        let a = geo::Vec3f::new(1.0, 1.0, 0.0);
        let b = geo::Vec3f::new(1.0, 2.0, 0.0);
        let c = geo::Vec3f::new(0.0, 2.0, 0.0);
        let triangle = Triangle::new(a, b, c);
        let verts = triangle.vertices();
        assert_eq!(verts[0], &geo::Vec3f::new(1.0, 1.0, 0.0));
        assert_eq!(verts[1], &geo::Vec3f::new(1.0, 2.0, 0.0));
        assert_eq!(verts[2], &geo::Vec3f::new(0.0, 2.0, 0.0));
    }

    #[test]
    fn line_points() {
        let a = geo::Vec3i::new(1, 2, 3); let b = geo::Vec3i::new(4, 5, 6);
        let line = Line::new(a, b);
        let points = line.vertices();
        assert_eq!(points[1], &geo::Vec3i::new(1, 2, 3));
        assert_eq!(points[0], &geo::Vec3i::new(4, 5, 6));
    }

    #[test]
    fn line_iterate() {
        let line = Line::new(geo::)
    }

}