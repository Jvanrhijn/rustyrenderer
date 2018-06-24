use std::mem;
use std::cmp;
mod geo;
use geo::Vector;
mod model;
extern crate image;


fn line(mut x0: u32, mut y0: u32, mut x1: u32, mut y1: u32,
        img: &mut image::RgbImage, pix: image::Rgb<u8>) {
    let steep = (x1 as i32 - x0 as i32).abs() < (y1 as i32 - y0 as i32).abs();
    if steep {
        mem::swap(&mut x0, &mut y0);
        mem::swap(&mut y1, &mut x1);
    }
    if x0 > x1 {
        mem::swap(&mut x0, &mut x1);
        mem::swap(&mut y0, &mut y1);
    }
    for x in x0..=x1 {
        let t: f64 = (x - x0) as f64 / ((x1 - x0) as f64);
        let y = ((y0 as f64)*(1. - t) + (y1 as f64)*t) as u32;
        if steep {
            img.put_pixel(y, x, pix);
        } else {
            img.put_pixel(x, y, pix);
        }
    }
}

fn triangle(face: &geo::Vec3i, model: &model::Obj, img: &mut image::RgbImage, pix: image::Rgb<u8>) {
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
        line(x0, y0, x1, y1, img, pix);
    }
}


fn main() {
    let imgx = 1000;
    let imgy = 1000;

    let mut imgbuf = image::RgbImage::new(imgx, imgy);

    let head = model::Obj::from_file("obj/diablo3_pose.obj").unwrap();

    for i in 0..head.nfaces {
        let face = head.face(i);
        triangle(&face, &head, &mut imgbuf, image::Rgb([255, 255, 255]));
    }
    image::imageops::flip_vertical(&imgbuf).save("test.png").expect("Failed to save image");
}

