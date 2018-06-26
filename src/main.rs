mod geo;
mod model;
mod obj;
use model::Polygon;
extern crate image;


fn triangle(face: &geo::Vec3i, model: &obj::Obj, img: &mut image::RgbImage) {
    let (imgx, imgy) = img.dimensions();
    let (imgx, imgy) = (imgx - 1, imgy - 1);
    let face = vec![face.x, face.y, face.z];
    for j in 0..3 {
        let v0 = model.vert(face[j] as usize);
        let v1 = model.vert(face[(j+1) % 3] as usize);
        let x0 = ((v0.x + 1. )*0.5*(imgx as f64)) as u32;
        let y0 = ((v0.y + 1. )*0.5*(imgy as f64)) as u32;
        let x1 = ((v1.x + 1.)*0.5*(imgx as f64)) as u32;
        let y1 = ((v1.y + 1.)*0.5*(imgy as f64)) as u32;
        let start = geo::Vec3::new(x0, y0, 0);
        let end = geo::Vec3::new(x1, y1, 0);
        let line = model::Line::new(&start, &end);

        line.draw(img, &[255, 255, 255]);
    }
}


fn main() {
    let imgx = 1000;
    let imgy = 1000;

    let mut imgbuf = image::RgbImage::new(imgx, imgy);

    let head = obj::Obj::from_file("obj/african_head.obj").unwrap();

    head.draw_wireframe(&mut imgbuf, &[255, 255, 255]);
    image::imageops::flip_vertical(&imgbuf).save("test.png").expect("Failed to save image");
}

