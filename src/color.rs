#![allow(dead_code)]
const VIOLET:   image::Rgb<u8> = image::Rgb([170, 063, 252]);
const SKYBLUE:  image::Rgb<u8> = image::Rgb([063, 110, 252]);
const PURPLE:   image::Rgb<u8> = image::Rgb([065, 000, 112]);
const POWDPINK: image::Rgb<u8> = image::Rgb([252, 148, 212]);
const HOTPINK:  image::Rgb<u8> = image::Rgb([255, 000, 255]);
const RED:      image::Rgb<u8> = image::Rgb([255, 000, 000]);
const DARKBLUE: image::Rgb<u8> = image::Rgb([020, 020, 065]);
const LITBLUE:  image::Rgb<u8> = image::Rgb([160, 160, 230]);
//const VIOLET: image::Rgb<u8> = image::Rgb([0,0,0]);
//const VIOLET: image::Rgb<u8> = image::Rgb([0,0,0]);
//const VIOLET: image::Rgb<u8> = image::Rgb([0,0,0]);
//const VIOLET: image::Rgb<u8> = image::Rgb([0,0,0]);
//const VIOLET: image::Rgb<u8> = image::Rgb([0,0,0]);
//const VIOLET: image::Rgb<u8> = image::Rgb([0,0,0]);
//const VIOLET: image::Rgb<u8> = image::Rgb([0,0,0]);
//const VIOLET: image::Rgb<u8> = image::Rgb([0,0,0]);

fn transition(state: &image::Rgb<u8>, target: image::Rgb<u8>, ratio: f64) -> image::Rgb<u8>
{
    let mut color: image::Rgb<u8> = image::Rgb([0,0,0]);

    for n in 0..3
    {
        color[n] = (state[n] as f64  + ((ratio * (target[n] as f64 - color[n] as f64)))) as u8;
    }

    return color;
}

pub fn test_transition_color(i: u32) -> image::Rgb<u8>
{
    let palette: Vec<image::Rgb<u8>> = vec![DARKBLUE, SKYBLUE, LITBLUE];
    let count = i % 300;
    let mut state: image::Rgb<u8> = palette[(count / 100) as usize];
  
         if count < 100 { state = transition(&state, palette[1],  count        as f64 / 100.0) }
    else if count < 200 { state = transition(&state, palette[2], (count - 100) as f64 / 100.0) }
    else                { state = transition(&state, palette[0], (count - 200) as f64 / 100.0) }

    return state;
}

pub fn test_color(i: u32) -> image::Rgb<u8>
{
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
