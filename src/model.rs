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

    fn inside(&self, point: vec::Vec<&geo::Vec3<T>>) -> bool;
}

pub struct Line<T> {
    start: geo::Vec3<T>,
    end: geo::Vec3<T>,
}

impl<T> Line<T>
    where T: geo::Number<T>
{

    pub fn new(start: geo::Vec3<T>, end: geo::Vec3<T>) -> Line<T> {
        Line{start, end}
    }

    fn rasterize(&self, img: &image::RgbImage) -> Line<u32> {
        let (imgx, imgy) = img.dimensions();
        let (imgx, imgy) = (imgx-1, imgy-1);
        let start = geo::Vec3::<u32>::new(((self.start.x.to_f64().unwrap() + 1.)*0.5*(imgx as f64)) as u32,
                                          ((self.start.y.to_f64().unwrap() + 1.)*0.5*(imgy as f64)) as u32, 0);
        let end = geo::Vec3::<u32>::new(((self.end.x.to_f64().unwrap() + 1.)*0.5*(imgx as f64)) as u32,
                                        ((self.end.y.to_f64().unwrap() + 1.)*0.5*(imgy as f64)) as u32, 0);
        Line::new(start, end)
    }

    fn iter(&self) -> LineIterator {
        LineIterator::new(self)
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
        let mut y = y0;
        let mut x = x0;
        for pixel in self.iter() {
            let geo::Vec3i{x, y, z: _} = pixel;
            img.put_pixel(x as u32, y as u32, image::Rgb::<u8>(*color));
        }
    }

    fn draw_filled(&self, img: &mut image::RgbImage, color: &[u8; 3]) {

    }

    fn vertices(&self) -> vec::Vec<&geo::Vec3<T>> {
        vec![&self.start, &self.end]
    }

    fn inside(&self, point: geo::Vec3<T>) -> bool {
        false
    }

}

struct LineIterator
{
    line: Line<u32>,
    dx: i32,
    dy: i32,
    derror: i32,
    error: i32,
    x: i32,
    y: i32,
    steep: bool,
    pixel: geo::Vec3<u32>,
}

impl LineIterator
{
    pub fn new<T>(line: &Line<T>) -> LineIterator
        where T: geo::Number<T>
    {
        let pixel = geo::Vec3::<u32>::new(line.start.x.to_u32().unwrap(),line.start.y.to_u32().unwrap(), 0);
        let Line{start, end} = line;
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
        let oriented_line = Line::new(geo::Vec3::<u32>::new(x0, y0, 0),
                                                geo::Vec3::<u32>::new(x1, y1, 0));
        LineIterator{line: oriented_line, dx, dy, derror, error: 0,
                     x: x0 as i32, y: y0 as i32, steep, pixel}
    }
}

impl Iterator for LineIterator
{
    type Item = geo::Vec3i;

    fn next(&mut self) -> Option<geo::Vec3i> {
        self.error += self.derror;
        if self.error > self.dx {
            self.y += if self.dy > 0 { 1 } else { -1 };
            self.error -= self.dx * 2;
        }
        self.x += 1;
        let (x, y) = match self.steep {
            false => (self.x, self.y),
            true => (self.y, self.x)
        };
        if self.x <= self.line.end.x as i32 {
            Some(geo::Vec3i::new(x, y, 0))
        } else {
                None
        }
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

    fn inside(&self, point: geo::Vec3<T>) -> bool {
        false
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
        assert_eq!(points[0], &geo::Vec3i::new(1, 2, 3));
        assert_eq!(points[1], &geo::Vec3i::new(4, 5, 6));
    }

    #[test]
    fn line_iterate() {
        let line = Line::new(geo::Vec3::<u64>::new(0, 0, 0),
                             geo::Vec3::<u64>::new(9, 9, 0));
        let image = image::RgbImage::new(10, 20);
        let line_iter = LineIterator::new(&line);
        let mut x = 0;
        let mut y = 0;
        for (i, pixel) in line.iter().enumerate() {
            assert_eq!(pixel, geo::Vec3i::new((i+1) as i32, (i+1) as i32, 0));
        }
    }
}