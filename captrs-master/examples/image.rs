extern crate captrs;
extern crate shuteye;

use captrs::*;
use shuteye::sleep;
use std::env;
use std::fs::File;
use std::io::Cursor;
use std::path::Path;
use std::time::Duration;

use image::*;
fn main() {
    let file: String = if env::args().count() == 2 {
        env::args().nth(1).unwrap()
    } else {
        panic!("Please enter a file")
    };

    // Use the open function to load an image from a Path.
    // ```open``` returns a dynamic image.
    let im: image::DynamicImage = image::open(&Path::new(&file)).unwrap();

    let mut capturer: Capturer = Capturer::new(1).unwrap();
    let (w, h) = capturer.geometry();
    let size: u64 = w as u64 * h as u64;
    let ps: Vec<dxgcap::BGRA8> = capturer.capture_frame().unwrap();
    sleep(Duration::from_millis(80));
    let ps = capturer.capture_frame().unwrap();
    let buffer = {
        let mut temp = vec![];
        for Bgr8 { r, g, b, a } in ps.as_slice() {
            temp.push(*r);
            temp.push(*g);
            temp.push(*b);
            temp.push(*a);
        }
        temp
    };
    let mut img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_fn(w, h, |x, y| {
        let index: usize = 4 * (x + w * y) as usize;
        image::Rgba([
            buffer[index + 0],
            buffer[index + 1],
            buffer[index + 2],
            buffer[index + 3],
        ])
    });
    // let mut img: Option<ImageBuffer<_, Vec<Rgba<u8>>>> = image::ImageBuffer::from_vec(w, h, buffer);
    // let mut reader = io::Reader::new(Cursor::new(buffer))
    //     .with_guessed_format()
    //     .expect("Cursor io never fails");
    // println!("{:?}", (img));
    let fouta: &mut File = &mut File::create(&Path::new(&format!("{}lol.png", file))).unwrap();

    // Write the contents of this image to the Writer in PNG format.
    // let rgba = open("path/to/some.png").unwrap().into_rgba8();
    let gray = DynamicImage::ImageRgba8(img);
    gray.write_to(fouta, ImageFormat::Png).unwrap();

    // The dimensions method returns the images width and height
    println!("dimensions {:?}", im.dimensions());

    // The color method returns the image's ColorType
    println!("{:?}", im.color());

    let fout: &mut File = &mut File::create(&Path::new(&format!("{}.jpeg", file))).unwrap();

    // Write the contents of this image to the Writer in PNG format.
    im.write_to(fout, ImageFormat::Jpeg).unwrap();
}
