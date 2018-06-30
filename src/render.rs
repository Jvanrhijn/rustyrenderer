use std;
use std::vec;
use image;
use geo;
use model;
use model::Polygon;
use obj;

pub struct Scene<'a> {
    objects: Vec<obj::Obj>,
    light_dir: geo::Vec3f,
    img: &'a mut image::RgbImage,
}

impl<'a> Scene<'a> {

    pub fn new(objects: Vec<obj::Obj>, img: &'a mut image::RgbImage) -> Scene {
       Scene{objects, light_dir: geo::Vec3f::new(0., 0., -1.), img}
    }

    pub fn add_object(&mut self, obj: obj::Obj) {
        self.objects.push(obj);
    }

    pub fn light_direction(&mut self, x: f64, y: f64, z: f64) {
        self.light_dir = geo::Vec3f::new(x, y, z).normalize();
    }

    pub fn draw(&mut self) {
        for obj in (&self.objects).into_iter() {
            ObjRenderer::new(obj).draw_lit(self.img, self.light_dir);

        }
    }

    pub fn save(self, path: &str) -> Result<(), std::io::Error>  {
        image::imageops::flip_vertical(self.img).save(path)
    }

}

pub struct ObjRenderer<'a> {
    obj: &'a obj::Obj,
}

impl<'a> ObjRenderer<'a> {

    pub fn new(obj: &'a obj::Obj) -> ObjRenderer {
        ObjRenderer{obj}
    }

    pub fn draw_wireframe(&self, mut img: &mut image::RgbImage, rgb: &[u8; 3]) {
        let (imgx, imgy) = img.dimensions();
        let (imgx, imgy) = (imgx-1, imgy-1);
        for face in self.obj.faces.iter() {
            self.obj.get_triangle(face).draw_filled(img, rgb);
        }
    }

    pub fn draw_lit(&self, mut img: &mut image::RgbImage, light_dir: geo::Vec3f) {
        let (imgx, imgy) = img.dimensions();
        let (imgx, imgy) = (imgx-1, imgy-1);
        for face in self.obj.faces.iter() {
            let triangle = self.obj.get_triangle(face);
            let intensity = obj::Obj::light_intensity(&triangle, light_dir);
            if intensity > 0. {
                let color = [(255.*intensity) as u8, (255.*intensity) as u8, (255.*intensity) as u8];
                triangle.draw_filled(img, &color);
            }
        }
    }

}