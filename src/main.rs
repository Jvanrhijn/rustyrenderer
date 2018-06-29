mod geo;
mod model;
mod obj;
use model::Polygon;
extern crate image;

fn main() {
    let imgx = 1000;
    let imgy = 1000;

    let mut imgbuf = image::RgbImage::new(imgx, imgy);

    let head = obj::Obj::from_file("obj/diablo3_pose.obj").unwrap();

    head.draw_lit(&mut imgbuf, geo::Vec3f::new(0., 0., -1.));
    image::imageops::flip_vertical(&imgbuf).save("test.png").expect("Failed to save image");
}

