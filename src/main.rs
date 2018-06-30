mod geo;
mod model;
mod obj;
mod render;
use std::vec::{Vec};
extern crate image;

fn main() {
    let imgx = 800;
    let imgy = 800;

    let mut imgbuf = image::RgbImage::new(imgx, imgy);

    let mut scene = render::Scene::new(Vec::<obj::Obj>::new(), &mut imgbuf);
    scene.add_object(obj::Obj::from_file("obj/african_head.obj").unwrap());

    scene.draw();
    scene.save("test.png").expect("Failed to save image");
}

