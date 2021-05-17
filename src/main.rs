extern crate image;
extern crate num_complex;

mod color;
mod frac;

use std::{env, process::exit};
use frac::*;


fn print_usage()
{
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

fn parse_args() -> Details
{
    let mut details= Details
    {
      frac_type: "m".to_string(),
           imgx: 1000,
           imgy:1000,
         scalex: 3.0,
         scaley: 3.0,
        centerx: 0.0,
        centery: 0.0,
           imax: 1000,
       filename: "fractal".to_string(), 
    };

    let args: Vec<String> = env::args().collect();

    if args.len() != 7
    {
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
fn validate_details(details: &Details)
{
    if details.imgx > 99999|| details.imgy > 99999
    {
        println!("Resolution exceeds upper bounds");
        print_usage();
        exit(1);
    }
    if details.scalex <= 0.0 || details.scaley <= 0.0
    {
        println!("Invalid scale value");
        print_usage();
        exit(1);
    }
}

/**
 * Print helpful stuff to stdout
 */
fn print_details(details: &Details)
{
    println!("┌────────────────────────────────┐");
    println!("│Fractal type: {}                 │", details.frac_type);
    println!("│Dimensions: {0:>5}x{1:<5} (\\___/) │", details.imgx, details.imgy);
    println!("│Scale: {0:>.4}:{1:<.4}    (='.'=) │", details.scalex, details.scaley);
    println!("│Center: ({0:<.2},{1:<.2})   (\")___(\")│", details.centerx, details.centery);
    println!("│Maximum iterations: {0:<10}  │", details.imax);
    println!("│Filename: {:<20}  │", details.filename);
    println!("└────────────────────────────────┘");
}

fn main()
{
    let fractal = parse_args();
    validate_details(&fractal);
    print_details(&fractal);
    create_fractal(&fractal);
    animate_zoom(fractal, 3000, 0.98);
}