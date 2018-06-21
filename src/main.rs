use std::mem;
mod geo;
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
    let imgx = 100;
    let imgy = 100;

    let mut imgbuf = image::RgbImage::new(imgx, imgy);

    line(90, 20, 80, 90, &mut imgbuf, image::Rgb([255, 255, 255]));
    line(20, 13, 40, 80, &mut imgbuf, image::Rgb([255, 0, 0]));
    line(80, 40, 13, 20, &mut imgbuf, image::Rgb([255, 0, 0]));
    imgbuf.save("test.png").unwrap();
}

