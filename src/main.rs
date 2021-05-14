extern crate image;
extern crate num_complex;

mod color;

use crate::color::*;
use std::{env, process::exit, u8};
use std::fs::create_dir;

struct Details {
    frac_type: String,
    imgx: u32,
    imgy: u32,
    scalex: f64,
    scaley: f64,
    centerx: f64,
    centery: f64,
    imax: u32,
    filename: String,
}

fn print_usage() {
    println!("Proper Usage: <exe> [type] [resolution] [scale] [center] [imax] [filename]");
    println!("===========================================================================");
    println!("|     [type]: m - Mandelbrot set      | [resolution]: Output image size   |");
    println!("|             j - Julia set           |              Formatted as [n]x[n] |");
    println!("|             b - Burning Ship set    |                 ex. 500x700       |");
    println!("|                                     |                                   |");
    println!("===========================================================================");
    println!("===========================================================================");
    println!("|     [scale]: Float value represe-   | [center]: Where the image will be |");
    println!("|             nting width of view of  |          centered on the complex  |");
    println!("|             the complex plane       |          plane                    |");
    println!("|                                     |          Formatted as [x],[y]     |");
    println!("|                ex. 5.0              |                 ex. 0.3,5.0       |");
    println!("===========================================================================");
    println!("===========================================================================");
    println!("|     [imax]: The escape time for the | [filename]: Output filename       |");
    println!("|            algorithm                |                                   |");
    println!("|                                     |                                   |");
    println!("|                ex. 3000             |                 ex. fractal.png   |");
    println!("===========================================================================");
}

fn parse_args() -> Details {
    let mut details= Details { frac_type: "m".to_string(),
                                       imgx: 1000,
                                       imgy:1000,
                                       scalex: 3.0,
                                       scaley: 3.0,
                                       centerx: 0.0,
                                       centery: 0.0,
                                       imax: 1000,
                                       filename: "fractal".to_string(), };

    let args: Vec<String> = env::args().collect();

    if args.len() != 7 {
        println!("Incorrect number of arguments. \n");
        print_usage();
        exit(1);
    }

    let mut size = args[2].split('x');
    let mut center = args[4].split(',');

    // Stuff console arguments into struct
    details.frac_type = args[1].to_string();
    details.imgx = size.next().unwrap().parse::<u32>().expect("Invalid x resolution value");
    details.imgy = size.next().unwrap().parse::<u32>().expect("Invalid y resolution value");
    details.scalex = args[3].parse::<f64>().expect("Invalid provided scale value");
    details.scaley = details.scalex * (details.imgy as f64 / details.imgx as f64);
    details.centerx = center.next().unwrap().parse::<f64>().expect("Invalid x center value");
    details.centery = center.next().unwrap().parse::<f64>().expect("Invalid y center value");
    details.imax = args[5].parse::<u32>().expect("Invalid imax value");
    details.filename = args[6].to_string();

    return details;
}

/**
 * Check for bad things happening //TODO check for more bad stuff
 */
fn validate_details(details: &Details) {
    if details.imgx > 99999|| details.imgy > 99999 {
        println!("Resolution exceeds upper bounds");
        print_usage();
        exit(1);
    }
    if details.scalex <= 0.0 || details.scaley <= 0.0 {
        println!("Invalid scale value");
        print_usage();
        exit(1);
    }
}

/**
 * Print helpful stuff to stdout
 */
fn print_details(details: &Details) {

    println!("┌────────────────────────────────┐");
    println!("│Fractal type: {}                 │", details.frac_type);
    println!("│Dimensions: {0:>5}x{1:<5} (\\___/) │", details.imgx, details.imgy);
    println!("│Scale: {0:>.4}:{1:<.4}    (='.'=) │", details.scalex, details.scaley);
    println!("│Center: ({0:<.2},{1:<.2})   (\")___(\")│", details.centerx, details.centery);
    println!("│Maximum iterations: {0:<10}  │", details.imax);
    println!("│Filename: {:<20}  │", details.filename);
    println!("└────────────────────────────────┘");
}

/**   /   \    /   \   /
 *   |    O|  |    O|  --
 *    \   /    \   /   \
 */
fn animate_zoom(mut fractal: Details, frames: u32, rate: f64) {
    if frames > 99999 { 
        println!("Exceeded frame limit.");
        exit(1);
    }

    let dirname: String = fractal.filename;
    create_dir(&dirname).expect("Unable to create animation directory");

    for n in 0..frames {
        fractal.filename = format!("/{}/{:#05}.png", &dirname, n); 
        // TODO some code to tighten boundries and shiddd
        create_fractal(&fractal);
    }
}

/**
 * Draws and saves a fractal when provided with a &Details struct.
 */
fn create_fractal(fractal: &Details) {
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
                print_usage();
                exit(1); 
               },
    };

    let pick_color: fn(u32) -> image::Rgb<u8>;
    match "b" {
        "x" => { pick_color = violet_color; },
        "b" => { pick_color = test_color;   },
        _ => { exit(1); },
    };

    // Apply algorithm to all pixels/coordinates
    for x in 0..fractal.imgx {
        for y in 0..fractal.imgy {
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
fn mandelbrot_iter(fractal: &Details, cx: f64, cy: f64) -> u32 {
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
fn julia_iter(fractal: &Details, zx: f64, zy: f64) -> u32 {
    // TODO: Feed c value from details struct
    //let c = num_complex::Complex::new(fractal.cx, fractal.cy);
    let c = num_complex::Complex::new(-0.4, 0.6);
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
fn burning_iter(fractal: &Details, cx: f64, cy: f64) -> u32 {
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

fn main() {
    let mut fractal = parse_args();
    validate_details(&fractal);
    print_details(&fractal);
    create_fractal(&fractal);
}

