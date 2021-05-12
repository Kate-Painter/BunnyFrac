extern crate image;
extern crate num_complex;
use std::{env, process::exit, u8};

struct Details {
    frac_type: String,
    imgx: u32,
    imgy: u32,
    scalex: f32,
    scaley: f32,
    centerx: f32,
    centery: f32,
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
    details.scalex = args[3].parse::<f32>().unwrap();
    details.scaley = details.scalex * (details.imgy as f32 / details.imgx as f32);
    details.centerx = center.next().unwrap().parse::<f32>().unwrap();
    details.centery = center.next().unwrap().parse::<f32>().unwrap();
    details.imax = args[5].parse::<u32>().unwrap();
    details.filename = args[6].to_string();

    return details;
}

fn print_details(details: &Details) {
    println!("Fractal type: {}", details.frac_type);
    println!("Dimensions: {}x{}", details.imgx, details.imgy);
    println!("Scale: {}:{}", details.scalex, details.scaley);
    println!("Filename: {}", details.filename);
}

fn pick_color(i: u32, imax: u32) -> image::Rgb<u8> {
    let ratio = (i as f32 % 500 as f32) / 500 as f32;
    let r = 100 + (ratio * 100.0) as u8;
    let g = 0   + (ratio * 100.0) as u8;
    let b = 100 + (ratio * 120.0) as u8;
    return image::Rgb([r,g,b]);
}
fn create_mandelbrot(fractal: &Details) {
    let scalefx = fractal.scalex / fractal.imgx as f32;
    let scalefy = fractal.scaley / fractal.imgy as f32;

    let mut imgbuf = image::ImageBuffer::new(fractal.imgx, fractal.imgy);

    for x in 0..fractal.imgx {
        for y in 0..fractal.imgy {
            let cx = x as f32 * scalefx - (fractal.scalex / 2.0) + fractal.centerx;
            let cy = y as f32 * scalefy - (fractal.scaley / 2.0) + fractal.centery;

            let c = num_complex::Complex::new(cx, cy);
            let mut z = num_complex::Complex::new(0.0, 0.0);

            let mut i: u32 = 0;
            while i < fractal.imax && z.norm() <= 2.0 {
                z = c * c + z;
                i += 1;
            }

            let pixel = imgbuf.get_pixel_mut(x, y);
            let image::Rgb(_data) = *pixel;
            if i == fractal.imax {
                *pixel = image::Rgb([0, 0, 0]);
            }
            else {
                *pixel = pick_color(i, fractal.imax);
            }
        }
    }

    imgbuf.save(&fractal.filename).unwrap();
}
fn main() {
    let fractal = parse_args();
    print_details(&fractal);

    match &fractal.frac_type as &str {
        "m" => create_mandelbrot(&fractal),
        //"j" => create_julia(&fractal),
        //"b" => create_brokenship(&fractal),
        _ => {
            println!("Unrecognized fractal type.\n");
            print_usage();
            exit(1);
        }
    };
}

