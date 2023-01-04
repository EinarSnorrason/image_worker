use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};use image::{Rgb};
use rayon::prelude::*;
use std::time::Instant;
use num_complex::{Complex, ComplexFloat};

fn calculate_pixel(x:u32, y:u32, pixel: &mut Rgb<u8>, imgx: u32, imgy: u32, centrex: f32, centrey:f32){


    let scalex = 3.0 / imgx as f32;
    let scaley = 3.0 / imgy as f32;

    let cy = y as f32 * scaley - 1.5 +centrex;
    let cx = x as f32 * scalex - 1.0 + centrey;

    let z0 = Complex::new(-cx, cy);
    let mut z = z0.clone();

    let mut i = 0;
    let max_i = 1000;
    let scalei :f32= max_i as f32 / 255.0;
    let mut t_z = z.clone();
    while i < max_i && z.norm() <= 2.0 {
        //t_z = Complex::new(z.re.abs() , z.im.abs());
        //z = t_z * t_z + z0;
        z = z*z+z0;
        i += 1;
    }
    if i>=max_i{
        *pixel = Rgb([0,0,0]);
    }
    else{
        *pixel = Rgb([0, (i as f32*scalei) as u8,  0]);
    }
}

fn main() {
    let imgx = 1440;
    let imgy = 1440;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf =image::ImageBuffer::new(imgx, imgy);

    let now = Instant::now();
    imgbuf.enumerate_pixels_mut()
        .par_bridge()
        .for_each(|(x,y,pixel)|
            calculate_pixel(x,y,pixel,imgx,imgy,0.0,0.0));
    println!("Parallel: {}",now.elapsed().as_millis());

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("fractal.png").unwrap();

}