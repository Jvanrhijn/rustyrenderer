use std::path::Path;
use std::vec;
use geo;
extern crate tobj;

pub struct Obj {
    pub nvert: usize,
    pub nfaces: usize,
    pub vertices: vec::Vec<geo::Vec3f>,
    mesh : tobj::Mesh,
}

impl Obj {
    pub fn from_file(fpath: &str) -> Obj {
        let objf = tobj::load_obj(&Path::new(fpath));
        let (models, _materials) = objf.expect("Obj file is corrupted");
        let mesh = models[0].mesh.clone();
        let nvert = mesh.positions.len()/3;
        let nfaces = mesh.indices.len();
        let mut vertices = vec::Vec::<geo::Vec3f>::new();
        for i in 0..nvert {
            println!("{} {} {} {}", i, &mesh.positions[i*3], &mesh.positions[i*3+1], &mesh.positions[i*3+2]);
            let x = mesh.positions[i*3].clone().into();
            let y = mesh.positions[i*3+1].clone().into();
            let z = mesh.positions[i*3+2].clone().into();
            vertices.push(geo::Vec3f::new(x, y, z));
        }
        Obj{nvert, nfaces, vertices, mesh}
    }

    pub fn face(self, i: usize) -> vec::Vec<u32> {
        vec![self.mesh.indices[3*i], self.mesh.indices[3*i+1], self.mesh.indices[3*i+2]]
    }

    pub fn vert(self, i: usize) -> geo::Vec3f {
        self.vertices[i].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}

