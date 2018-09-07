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
    scene.add_object(obj::Obj::from_file("obj/diablo3_pose.obj").unwrap()
        .load_texture("obj/textures/diablo3_pose_diffuse.tga"));
    scene.light_direction(0., 0., -1.);
    scene.draw();
    scene.save("test.png").expect("Failed to save image");
}

