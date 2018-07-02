use std::cmp;
use std::vec::{Vec};
use std::vec;
use std::mem;
extern crate num;
use geo;
use geo::Vector;
use image;

pub trait Polygon<T>
{
    fn draw(&self, img: &mut image::RgbImage, color: &[u8; 3]);

    fn draw_filled(&self, img: &mut image::RgbImage, color: &[u8; 3], zbuf: &mut Vec<f64>);

    fn inside(&self, point: &geo::Vec3<T>) -> bool;

    fn bounding_box(&self, dimx: u32, dimy: u32) -> Line<i32>;
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

    fn rasterize(&self, xdim: u32, ydim: u32) -> Line<u32> {
        let (start, end) = (self.start.to_f64(), self.end.to_f64());
        let start = geo::Vec3::<u32>::new(((start.x + 1.)*0.5*(xdim as f64)) as u32,
                                          ((start.y + 1.)*0.5*(ydim as f64)) as u32, 0);
        let end = geo::Vec3::<u32>::new(((end.x + 1.)*0.5*(xdim as f64)) as u32,
                                        ((end.y + 1.)*0.5*(ydim as f64)) as u32, 0);
        Line::new(start, end)
    }

    fn vertices(&self) -> [&geo::Vec3<T>; 2] {
        [&self.start, &self.end]
    }


}

impl<T> Polygon<T> for Line<T>
    where T: geo::Number<T>
{

    fn draw(&self, img: &mut image::RgbImage, color: &[u8; 3]) {
        for pixel in self.into_iter() {
            let geo::Vec3i{x, y, z: _} = pixel;
            img.put_pixel(x as u32, y as u32, image::Rgb::<u8>(*color));
        }
    }

    fn draw_filled(&self, img: &mut image::RgbImage, color: &[u8; 3], zbuf: &mut Vec<f64>) {
        self.draw(img, color);
    }

    fn inside(&self, point: &geo::Vec3<T>) -> bool {
        let point = point.to_i32();
        for pixel in self.into_iter() {
            if pixel.x == point.x && pixel.y == point.y {
                return true;
            }
        }
        false
    }

    fn bounding_box(&self, _dimx: u32, _dimy: u32) -> Line<i32> {
        let Line{start, end} = self;
        let start = start.to_i32();
        let end = end.to_i32();
        Line{start, end}
    }

}

pub struct LineIterator
{
    line: Line<u32>,
    dx: i32,
    dy: i32,
    derror: i32,
    error: i32,
    x: i32,
    y: i32,
    steep: bool,
}

impl LineIterator
{
    pub fn new<T>(line: &Line<T>) -> LineIterator
        where T: geo::Number<T>
    {
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
                     x: x0 as i32, y: y0 as i32, steep}
    }
}

impl Iterator for LineIterator {
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

impl<'a, T> IntoIterator for &'a Line<T>
    where T: geo::Number<T>
{
    type Item = geo::Vec3i;
    type IntoIter = LineIterator;

    fn into_iter(self) -> Self::IntoIter {
        LineIterator::new(&self)
    }
}

pub struct Triangle<T> {
    a: geo::Vec3<T>,
    b: geo::Vec3<T>,
    c: geo::Vec3<T>,
    edges: [Line<T>; 3],
}

impl<T> Triangle<T>
    where T: geo::Number<T>
{

    pub fn new(a: geo::Vec3<T>, b: geo::Vec3<T>, c: geo::Vec3<T>) -> Triangle<T> {
        let ab = Line::new(a.clone(), b.clone());
        let bc = Line::new(b.clone(), c.clone());
        let ac = Line::new(a.clone(), c.clone());
        Triangle{a, b, c, edges: [ab, bc, ac]}
    }

    pub fn barycentric(&self, point: &geo::Vec3<f64>) -> geo::Vec3f {
        let (a, b, c) = (self.a.to_f64(), self.b.to_f64(), self.c.to_f64());
        let first = geo::Vec3::<f64>::new((&b-&a).x, (&c-&a).x, (&a-point).x);
        let second = geo::Vec3::<f64>::new((&b-&a).y, (&c-&a).y, (&a-point).y);
        let u = first.cross(&second);
        if u.z.abs() < 1. {
            geo::Vec3f::new(-1., 1., 1.)
        } else {
            geo::Vec3f::new(1.-(u.x+u.y)/u.z, u.y/u.z, u.x/u.z)
        }
    }

    fn rasterize(&self, dimx: u32, dimy: u32) -> Triangle<i32> {
        let mut vertices = vec::Vec::<geo::Vec3<i32>>::new();
        for vert in self.vertices().iter() {
            let vert = (*vert).to_f64();
            vertices.push(geo::Vec3::<i32>::new(((vert.x + 1.)*0.5*(dimx as f64)) as i32,
                                                ((vert.y + 1.)*0.5*(dimy as f64)) as i32, 0));
        }
        Triangle::new(vertices[0], vertices[1], vertices[2])
    }

    pub fn normal(&self) -> geo::Vec3f {
        let normal = (&self.c-&self.a).cross(&(&self.b-&self.a));
        normal.normalize()
    }

    fn vertices(&self) -> [&geo::Vec3<T>; 3] {
       [&self.a, &self.b, &self.c]
    }

}


impl<T> Polygon<T> for Triangle<T>
    where T: geo::Number<T> + num::ToPrimitive
{

    fn draw(&self, img: &mut image::RgbImage, color: &[u8; 3]) {
        let (imgx, imgy) = img.dimensions();
        let rast = self.rasterize(imgx-1, imgy-1);
        let (a, b, c) = match &rast.edges {
            [a, b, c] => (a, b, c),
            _         => unreachable!()
        };
        a.draw(img, color);
        b.draw(img, color);
        c.draw(img, color);
    }

    fn draw_filled(&self, img: &mut image::RgbImage, color: &[u8; 3], zbuf: &mut Vec<f64>) {
        let (imgx, imgy) = img.dimensions();
        let rast = self.rasterize(imgx, imgy);
        let Line{start: bbox_min, end: bbox_max} = self.bounding_box(imgx, imgy);
        let mut point = geo::Vec3::<f64>::new(0., 0., 0.);
        for x in bbox_min.x..bbox_max.x {
            for y in bbox_min.y..bbox_max.y {
                point.x = x as f64; point.y = y as f64;
                if rast.inside(&point.to_i32()) {
                    point.z = 0.;
                    let barycentric = self.barycentric(&point).as_vec();
                    for (i, vertex) in self.vertices().into_iter().enumerate() {
                        point.z += vertex.to_f64().z*barycentric[i];
                    }
                    if zbuf[(point.x + point.y*(imgx as f64)) as usize] < point.z {
                        zbuf[(point.x + point.y*(imgx as f64)) as usize] = point.z;
                        img.put_pixel(point.x as u32, point.y as u32, image::Rgb::<u8>(*color));
                    }
                }
            }
        }
    }

    fn inside(&self, point: &geo::Vec3<T>) -> bool {
        let geo::Vec3f{x, y, z} = self.barycentric(&point.to_f64());
        !(x < 0. || y < 0. || z < 0.)
    }

    fn bounding_box(&self, dimx: u32, dimy: u32) -> Line<i32> {
        let rast =  self.rasterize(dimx, dimy);
        let mut bbox_max = geo::Vec3::<i32>::new(0, 0, 0);
        let mut bbox_min = geo::Vec3::<i32>::new(dimx as i32 -1, dimy as i32 -1, 0);
        let clamp = geo::Vec2::<i32>::new(dimx as i32 -1, dimy as i32 -1);
        for vertex in rast.vertices().into_iter() {
            let vertex = vertex.to_i32();
            bbox_min.x = cmp::max(0, cmp::min(bbox_min.x, vertex.x));
            bbox_min.y = cmp::max(0, cmp::min(bbox_min.y, vertex.y));
            bbox_max.x = cmp::min(clamp.x, cmp::max(bbox_max.x, vertex.x));
            bbox_max.y = cmp::min(clamp.y, cmp::max(bbox_max.y, vertex.y));
        }
        Line{start: bbox_min, end: bbox_max}
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
        for (i, pixel) in line.into_iter().enumerate() {
            assert_eq!(pixel, geo::Vec3i::new((i+1) as i32, (i+1) as i32, 0));
        }
    }

    #[test]
    fn inside_triangle() {
        let a = geo::Vec3f::new(0.0, 0.0, 0.0);
        let b = geo::Vec3f::new(1.0, 0.0, 0.0);
        let c = geo::Vec3f::new(1.0, 1.0, 0.0);
        let triangle = Triangle::new(a, b, c);
        let not_inside = geo::Vec3f::new(-0.1, 0., 0.);
        let inside = geo::Vec3f::new(0.9, 0.1, 0.);
        assert!(!triangle.inside(&not_inside));
        assert!(triangle.inside(&inside));
    }

}