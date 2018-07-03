use std;
use std::io;
use std::vec;
use std::str::FromStr;
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;
extern crate rand;
use model;
use geo;
use geo::Vector;
use image;

pub struct Obj {
    pub nvert: usize,
    pub nfaces: usize,
    pub vertices: vec::Vec<geo::Vec3f>,
    pub faces: vec::Vec<geo::Vec3i>,
    texture: Option<image::DynamicImage>,
}

impl Obj {
    pub fn from_file(fpath: &str) -> io::Result<Obj> {
        let file = File::open(fpath)?;
        let buf_reader = BufReader::new(file);
        let mut vertices = vec::Vec::<geo::Vec3f>::new();
        let mut faces = vec::Vec::<geo::Vec3i>::new();
        for line in buf_reader.lines() {
            let line = line.unwrap();
            if line.len() < 3 {
                continue;
            }
            let prefix = &line[..2];
            match prefix {
                "v " => vertices.push(geo::Vec3f::from(&Obj::collect_vec::<f64>(&line))),
                "f " => faces.push(geo::Vec3i::from(&Obj::collect_face(&line))),
                _    => continue
            };
        }
        Ok(Obj{nvert: vertices.len(), nfaces: faces.len(), vertices, faces, texture: None})
    }

    pub fn load_texture(mut self, path: &str) -> Self {
        self.texture = Some(image::open(path).unwrap());
        self
    }

    pub fn face(&self, i: usize) -> geo::Vec3i {
        self.faces[i].clone()
    }

    pub fn vert(&self, i: usize) -> geo::Vec3f {
        self.vertices[i].clone()
    }

    fn collect_vec<T>(s: &str) -> [T; 3]
        where T: FromStr + geo::Number<T>,
              <T as std::str::FromStr>::Err : std::fmt::Debug
    {
        let v: Vec<T> = s[2..].split_whitespace().map(|x| x.parse::<T>().unwrap()).collect();
        [v[0].clone(), v[1].clone(), v[2].clone()]
    }

    fn collect_face(s: &str) -> [i32; 3] {
        let terms: Vec<&str> = s[2..].split_whitespace().collect();
        let mut vec = vec::Vec::<i32>::new();
        for i in 0..3 {
            let indices = terms[i].split("/")
                .map(|x| x.parse::<i32>().unwrap());
            vec.push(indices.collect::<vec::Vec<i32>>()[0]-1); // indices in wavefront start with 1
        }
        let array: [i32; 3] = [vec[0], vec[1], vec[2]];
        array
    }

    pub fn get_triangle(&self, face: &geo::Vec3<i32>) -> model::Triangle<f64> {
        let af = &self.vertices[face.x as usize];
        let bf = &self.vertices[face.y as usize];
        let cf = &self.vertices[face.z as usize];
        model::Triangle::new(*af, *bf, *cf)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_obj() {
        let obj = Obj::from_file("obj/african_head.obj").unwrap();
        assert_eq!(obj.nvert, 1258);
        assert_eq!(obj.nfaces, 2492);
        assert_eq!(obj.vert(0), geo::Vec3f::new(-0.000581696, -0.734665, -0.623267));
        assert_eq!(obj.face(0), geo::Vec3i::new(23, 24, 25));
    }

}

