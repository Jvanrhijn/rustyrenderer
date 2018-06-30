use std;
use geo::Vector;
use std::str::FromStr;
use std::io;
use std::io::BufReader;
use std::fs::File;
use std::vec;
use geo;
use std::io::prelude::*;
use model;
extern crate rand;

pub struct Obj {
    pub nvert: usize,
    pub nfaces: usize,
    pub vertices: vec::Vec<geo::Vec3f>,
    pub faces: vec::Vec<geo::Vec3i>,
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
                "v " => vertices.push(geo::Vec3f::from_vec(&Obj::collect_vec::<f64>(&line))),
                "f " => faces.push(geo::Vec3i::from_vec(&Obj::collect_face(&line))),
                _    => continue
            };
        }
        Ok(Obj{nvert: vertices.len(), nfaces: faces.len(), vertices, faces})
    }

    pub fn face(&self, i: usize) -> geo::Vec3i {
        self.faces[i].clone()
    }

    pub fn vert(&self, i: usize) -> geo::Vec3f {
        self.vertices[i].clone()
    }

    fn collect_vec<T>(s: &str) -> vec::Vec<T>
        where T: FromStr,
              <T as std::str::FromStr>::Err : std::fmt::Debug
    {
        s[2..].split_whitespace().map(|x| x.parse::<T>().unwrap()).collect()
    }

    fn collect_face(s: &str) -> vec::Vec<i32> {
        let terms: Vec<&str> = s[2..].split_whitespace().collect();
        let mut vec = vec::Vec::<i32>::new();
        for i in 0..3 {
            let indices = terms[i].split("/")
                .map(|x| x.parse::<i32>().unwrap());
            vec.push(indices.collect::<vec::Vec<i32>>()[0]-1); // indices in wavefront start with 1
        }
        vec
    }

    pub fn get_triangle(&self, face: &geo::Vec3<i32>) -> model::Triangle<f64> {
        let af = &self.vertices[face.x as usize];
        let bf = &self.vertices[face.y as usize];
        let cf = &self.vertices[face.z as usize];
        model::Triangle::new(*af, *bf, *cf)
    }

    pub fn light_intensity(triangle: &model::Triangle<f64>, direction: geo::Vec3f) -> f64 {
        let normal = triangle.normal();
        normal.dot(&direction.normalize())
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

