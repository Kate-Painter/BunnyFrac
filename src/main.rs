extern crate image;
extern crate num_complex;

use std::{env, process::exit, u8};

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

    details.frac_type = args[1].to_string();
    details.imgx = size.next().unwrap().parse::<u32>().unwrap();
    details.imgy = size.next().unwrap().parse::<u32>().unwrap();
    details.scalex = args[3].parse::<f64>().unwrap();
    details.scaley = details.scalex * (details.imgy as f64 / details.imgx as f64);
    details.centerx = center.next().unwrap().parse::<f64>().unwrap();
    details.centery = center.next().unwrap().parse::<f64>().unwrap();
    details.imax = args[5].parse::<u32>().unwrap();
    details.filename = args[6].to_string();

    return details;
}

fn print_details(details: &Details) {

    println!("┌────────────────────────────────┐");
    println!("│Fractal type: {}                 │", details.frac_type);
    println!("│Dimensions: {0:>5}x{1:<5}         │", details.imgx, details.imgy);
    println!("│Scale: {0:>.4}:{1:<.4}            │", details.scalex, details.scaley);
    println!("│Center: ({0:<5},{1:<5})           │", details.centerx, details.centery);
    println!("│Maximum iterations: {0:<10}  │", details.imax);
    println!("│Filename: {:<20}  │", details.filename);
    println!("└────────────────────────────────┘");
}

fn pick_color(i: u32) -> image::Rgb<u8> {
    let ratio = (i as f64 % 500 as f64) / 500 as f64;
    let r = 20  + (ratio * 235.0) as u8;
    let g = 20  - (ratio *  20.0) as u8;
    let b = 65  + (ratio * 190.0) as u8;
    return image::Rgb([r,g,b]);
}

fn create_mandelbrot(fractal: &Details) {
    let scalefx = fractal.scalex / fractal.imgx as f64;
    let scalefy = fractal.scaley / fractal.imgy as f64;

    let mut imgbuf = image::ImageBuffer::new(fractal.imgx, fractal.imgy);
    let mut time: f64 = 0.0;
    for x in 0..fractal.imgx {
        for y in 0..fractal.imgy {
            let cx = x as f64 * scalefx - (fractal.scalex / 2.0) + fractal.centerx;
            let cy = y as f64 * scalefy - (fractal.scaley / 2.0) + fractal.centery;

            let c = num_complex::Complex::new(cx, cy);
            let mut z = num_complex::Complex::new(0.0, 0.0);

            let mut i: u32 = 0;
            while i < fractal.imax && z.norm() <= 2.0 {
                z = z * z + c;
                i += 1;
            }

            let pixel = imgbuf.get_pixel_mut(x, y);
            let image::Rgb(_data) = *pixel;
            if i == fractal.imax {
                *pixel = image::Rgb([0, 0, 0]);
            }
            else {
                *pixel = pick_color(i);
            }
        }
        time += (1.0 / fractal.imgx as f64) * 100 as f64;
        print!("\r  >>>> {:.2}% done", time);
    }

    imgbuf.save(&fractal.filename).unwrap();
}

fn create_julia(fractal: &Details) {
    let scalefx = fractal.scalex / fractal.imgx as f64;
    let scalefy = fractal.scaley / fractal.imgy as f64;

    let mut imgbuf = image::ImageBuffer::new(fractal.imgx, fractal.imgy);
    let mut time: f64 = 0.0;
    for x in 0..fractal.imgx {
        for y in 0..fractal.imgy {
            let cx = x as f64 * scalefx - (fractal.scalex / 2.0) + fractal.centerx;
            let cy = y as f64 * scalefy - (fractal.scaley / 2.0) + fractal.centery;

            let c = num_complex::Complex::new(-0.7269, 0.1889);
            let mut z = num_complex::Complex::new(cx, cy);

            let mut i: u32 = 0;
            while i < fractal.imax && z.norm() <= 2.0 {
                z = z * z + c;
                i += 1;
            }
            
            let pixel = imgbuf.get_pixel_mut(x, y);
            let image::Rgb(_data) = *pixel;
            if i == fractal.imax {
                *pixel = image::Rgb([0, 0, 0]);
            }
            else {
                *pixel = pick_color(i);
            }
        }
        time += (1.0 / fractal.imgx as f64) * 100 as f64;
        print!("\r>>>> {:.2}% done", time);
    }
    imgbuf.save(&fractal.filename).unwrap();
}

fn create_burningship(fractal: &Details) {
    let scalefx = fractal.scalex / fractal.imgx as f64;
    let scalefy = fractal.scaley / fractal.imgy as f64;

    let mut imgbuf = image::ImageBuffer::new(fractal.imgx, fractal.imgy);
    let mut time: f64 = 0.0;
    for x in 0..fractal.imgx {
        for y in 0..fractal.imgy {
            let cx = x as f64 * scalefx - (fractal.scalex / 2.0) + fractal.centerx;
            let cy = y as f64 * scalefy - (fractal.scaley / 2.0) + fractal.centery;

            let c = num_complex::Complex::new(cx, cy);
            let mut z = num_complex::Complex::new(0.0, 0.0);

            let mut i: u32 = 0;
            while i < fractal.imax && z.norm() <= 2.0 {
                z.re = f64::abs(z.re);
                z.im = f64::abs(z.im);
                z = z * z + c;
                i += 1;
            }
            
            let pixel = imgbuf.get_pixel_mut(x, y);
            let image::Rgb(_data) = *pixel;
            if i == fractal.imax {
                *pixel = image::Rgb([0, 0, 0]);
            }
            else {
                *pixel = pick_color(i);
            }
        }
        time += (1.0 / fractal.imgx as f64) * 100 as f64;
        print!("\r>>>> {:.2}% done", time);
    }
    imgbuf.save(&fractal.filename).unwrap();
}

fn main() {
    let fractal = parse_args();
    print_details(&fractal);

    match &fractal.frac_type as &str {
        "m" => create_mandelbrot(&fractal),
        "j" => create_julia(&fractal),
        "b" => create_burningship(&fractal),
        _ => {
            println!("Unrecognized fractal type.\n");
            print_usage();
            exit(1);
        }
    };
}

