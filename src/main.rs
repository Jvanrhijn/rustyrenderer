mod geo;
mod model;
mod obj;
mod render;
use model::Polygon;
use std::vec::{Vec};
extern crate image;

fn main() {
    let imgx = 1000;
    let imgy = 1000;

    let mut imgbuf = image::RgbImage::new(imgx, imgy);

    let mut scene = render::Scene::new(Vec::<obj::Obj>::new(), &mut imgbuf);
    scene.add_object(obj::Obj::from_file("obj/diablo3_pose.obj").unwrap());

    scene.light_direction(1., 0., -1.);

    scene.draw();
    scene.save("test.png").expect("Failed to save image");
}

