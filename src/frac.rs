use crate::color::*;
use std::{process::exit, u8};
use std::fs::create_dir;
use std::process::Command;
use substring::Substring;

pub struct Details
{
    pub frac_type: String,
    pub imgx: u32,
    pub imgy: u32,
    pub scalex: f64,
    pub scaley: f64,
    pub centerx: f64,
    pub centery: f64,
    pub imax: u32,
    pub filename: String,
}

/**   /   \    /   \   /
 *   |    O|  |    O|  --
 *    \   /    \   /   \
 */
pub fn animate_zoom(mut fractal: Details, frames: u32, rate: f64)
{
    if frames > 99999
    { 
        println!("Exceeded frame limit.");
        exit(1);
    }

    let dirname: String = fractal.filename
                                .to_string()
                                .substring(0, fractal.filename
                                                    .to_string()
                                                    .find(".")
                                                    .unwrap())
                                .to_string();
    
    let file_ext: String = fractal.filename
                                .to_string()
                                .substring(fractal.filename
                                                .to_string()
                                                .find(".")
                                                .unwrap() + 1,
                                           fractal.filename.len())
                                .to_string(); 

    create_dir(&dirname).expect("Unable to create animation directory");

    for n in 0..frames
    {
        fractal.filename = format!("./{}/{:#05}.png", &dirname, n); 
        fractal.scalex = fractal.scalex * rate;
        fractal.scaley = fractal.scalex * (fractal.imgy as f64 / fractal.imgx as f64);
        create_fractal(&fractal);
    }

    Command::new("ffmpeg")
            .args(&["-r", "30",
                    "-f", "image2",
                    "-s", &format!("{}x{}", fractal.imgx, fractal.imgy),
                    "-i", &format!("./{}/%05d.{}", &dirname, &file_ext),
                    "-c:v", "libvpx",
                    "-b:v", "1M",
                    &format!("{}.webm", dirname)])
            .output()
            .expect("Failed to spawn ffmpeg procress");
}

/**
 * Draws and saves a fractal when provided with a &Details struct.
 */
pub fn create_fractal(fractal: &Details)
{
    // Find scale factor
    let scalefx = fractal.scalex / fractal.imgx as f64;
    let scalefy = fractal.scaley / fractal.imgy as f64;
    
    // Create image and set time to 0
    let mut imgbuf = image::ImageBuffer::new(fractal.imgx, fractal.imgy);
    let mut time: f64 = 0.0;

    // Choose appropriate set algorithm based on frac_type in struct
    let iterate: fn(&Details, f64, f64) -> u32;
    match &fractal.frac_type as &str {
        "m" => { iterate = self::mandelbrot_iter; },
        "j" => { iterate = self::julia_iter; },
        "b" => { iterate = self::burning_iter; },
        _   => {
                println!("Fractal type not found.");
                // TODO Should probably raise error instead
                exit(1); 
               },
    };

    let pick_color: fn(u32) -> image::Rgb<u8>;
    match "x" {
        "x" => { pick_color = test_transition_color; },
        "b" => { pick_color = test_color;   },
          _ => { exit(1); },
    };

    // Apply algorithm to all pixels/coordinates
    for x in 0..fractal.imgx
    {
        for y in 0..fractal.imgy
        {
            // Find C value based on our chosen pixel and scale factor
            let cx = x as f64 * scalefx - (fractal.scalex / 2.0) + fractal.centerx;
            let cy = y as f64 * scalefy - (fractal.scaley / 2.0) - fractal.centery;

            // Find i for the pixel
            let i = iterate(fractal, cx, cy);

            // Choose pixel in image
            let pixel = imgbuf.get_pixel_mut(x, y);
            let image::Rgb(_data) = *pixel;

            // Choose color based on i value
            if i == fractal.imax {
                *pixel = image::Rgb([0, 0, 0]);
            }
            else {
                *pixel = pick_color(i);
            }
        }
        // Update loading timer
        time += (1.0 / fractal.imgx as f64) * 100 as f64;
        print!("\r >>>> {:.2}% done", time);
    }

    imgbuf.save(&fractal.filename).unwrap();
}

/**
 * Finds out whether a provided C value is part of the mandelbrot set and returns the escape time as a u32.
 */
pub fn mandelbrot_iter(fractal: &Details, cx: f64, cy: f64) -> u32
{
    let c = num_complex::Complex::new(cx, cy);
    let mut z = num_complex::Complex::new(0.0, 0.0);

    let mut i: u32 = 0;
    while i < fractal.imax && z.norm() <= 2.0 {
        z = z * z + c;
        i += 1;
    }
    return i;
}

/**
 * Finds out whether a provided C value is part of the julia set and returns the escape time as a u32.
 */
pub fn julia_iter(fractal: &Details, zx: f64, zy: f64) -> u32
{
    // TODO: Feed c value from details struct
    //let c = num_complex::Complex::new(fractal.cx, fractal.cy);
    let c = num_complex::Complex::new(-0.8, 0.156);
    let mut z = num_complex::Complex::new(zx, zy);

    let mut i: u32 = 0;
    while i < fractal.imax && z.norm() <= 2.0 {
        z = z * z + c;
        i += 1;
    }
    return i;
}

/**
 * Finds out whether a provided C value is part of the burning-ship set and returns the escape time as a u32.
 */
pub fn burning_iter(fractal: &Details, cx: f64, cy: f64) -> u32
{
    let c = num_complex::Complex::new(cx, cy);
    let mut z = num_complex::Complex::new(0.0, 0.0);

    let mut i: u32 = 0;
    while i < fractal.imax && z.norm() <= 2.0 {
        z.re = f64::abs(z.re);
        z.im = f64::abs(z.im);
        z = z * z + c;
        i += 1;
    }
    return i;
}