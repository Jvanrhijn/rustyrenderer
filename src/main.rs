mod geo;
mod model;
mod obj;
use model::Polygon;
extern crate image;

fn main() {
    let imgx = 1000;
    let imgy = 1000;

    let mut imgbuf = image::RgbImage::new(imgx, imgy);

    let head = obj::Obj::from_file("obj/african_head.obj").unwrap();

    head.draw_wireframe(&mut imgbuf, &[255, 255, 255]);
    image::imageops::flip_vertical(&imgbuf).save("test.png").expect("Failed to save image");
    let line = model::Line::new(geo::Vec3::<u64>::new(0, 0, 0),
                         geo::Vec3::<u64>::new(9, 9, 0));
    let image = image::RgbImage::new(10, 20);
}

