pub fn violet_color(i: u32) -> image::Rgb<u8> {
    let ratio = (i as f64 % 500 as f64) / 500 as f64;
    let r = 20  + (ratio * 235.0) as u8;
    let g = 20  - (ratio *  20.0) as u8;
    let b = 65  + (ratio * 190.0) as u8;
    return image::Rgb([r,g,b]);
}

pub fn test_color(i: u32) -> image::Rgb<u8> {
    let ratio = (i as f64 % 1000 as f64) / 1000 as f64;
    let (r, g, b);
    if i < 500 {
        r = 18  + ((ratio * 2.0) * 125.0) as u8;
        g = 2   + ((ratio * 2.0) *  78.0) as u8;
        b = 18  + ((ratio * 2.0) * 160.0) as u8;
    }
    else {
        r = 143  + (((ratio - 0.5) * 2.0) * 107.0) as u8;
        g = 80   + (((ratio - 0.5) * 2.0) * 168.0) as u8;
        b = 178  + (((ratio - 0.5) * 2.0) *  75.0) as u8;
    }
    return image::Rgb([r,g,b]);
}
