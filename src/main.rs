use std::mem;
use std::cmp;
mod geo;
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


fn main() {
    let imgx = 500;
    let imgy = 500;

    let mut imgbuf = image::RgbImage::new(imgx, imgy);

    let head = model::Obj::from_file("obj/african_head.obj").unwrap();

    for i in 0..head.nfaces {
        let face = head.face(i);
        let face = vec![face.x, face.y, face.z];
        for j in 0..3 {
            let v0 = head.vert(face[j] as usize);
            let v1 = head.vert(face[(j+1) % 3] as usize);
            let x0 = cmp::min(((v0.x + 1.)*(imgx as f64)/2.) as u32, imgx-1);
            let y0 = cmp::min(((v0.y + 1.)*(imgy as f64)/2.) as u32, imgy-1);
            let x1 = cmp::min(((v1.x + 1.)*(imgx as f64)/2.) as u32, imgx-1);
            let y1 = cmp::min(((v1.y + 1.)*(imgy as f64)/2.) as u32, imgy-1);
            line(x0, y0, x1, y1, &mut imgbuf, image::Rgb([255, 255, 255]));
        }
    }
    imgbuf.save("test.png").unwrap();
}

