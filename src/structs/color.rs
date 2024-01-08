use std::{cmp::min, io::Error, num::ParseIntError};
use anyhow::bail;
use image::Rgba;

pub fn color_add(lhs: &Rgba<u8>, rhs: &Rgba<u8>) -> Rgba<u8> {
    // default to full-alpha, clamp rgb vals
    let new_r = min((lhs.0[0] as u64) + (rhs.0[0] as u64), 0xFF) as u8;
    let new_g = min((lhs.0[1] as u64) + (rhs.0[1] as u64), 0xFF) as u8;
    let new_b = min((lhs.0[2] as u64) + (rhs.0[2] as u64), 0xFF) as u8;

    Rgba ([ 
        new_r, 
        new_g, 
        new_b, 
        0xFF 
    ])
}

pub fn color_multiply(lhs: &Rgba<u8>, rhs: &Rgba<u8>) -> Rgba<u8> {
    let new_r = ((lhs.0[0] as u64) * (rhs.0[0] as u64) / 0xFF) as u8;
    let new_g = ((lhs.0[1] as u64) * (rhs.0[1] as u64) / 0xFF) as u8;
    let new_b = ((lhs.0[2] as u64) * (rhs.0[2] as u64) / 0xFF) as u8;

    Rgba ([ 
        new_r, 
        new_g, 
        new_b, 
        0xFF 
    ])
}

pub fn color_scale<T>(color: &Rgba<u8>, factor: T) -> Rgba<u8> 
    where T: Into<f64> + Copy
{
    let new_r = (color.0[0] as f64 * factor.into()) as u8;
    let new_g = (color.0[1] as f64 * factor.into()) as u8;
    let new_b = (color.0[2] as f64 * factor.into()) as u8;

    Rgba ([
        new_r,
        new_g,
        new_b,
        0xFF
    ])
}

pub fn color_from_hex(hex_string: &str) -> Result<Rgba<u8>, anyhow::Error> {
    if hex_string.len() == 7 {
        let r = u8::from_str_radix(&hex_string[1..3], 16);
        let g = u8::from_str_radix(&hex_string[3..5], 16);
        let b = u8::from_str_radix(&hex_string[5..7], 16);
        Ok(Rgba([r?, g?, b?, 0xFF]))
    } else if hex_string.len() == 9 {
        let r = u8::from_str_radix(&hex_string[1..3], 16);
        let g = u8::from_str_radix(&hex_string[3..5], 16);
        let b = u8::from_str_radix(&hex_string[5..7], 16);
        let a = u8::from_str_radix(&hex_string[7..9], 16);
        Ok(Rgba([r?, g?, b?, a?]))
    } else {
        bail!("Invalid hex string length ({})", hex_string)
    }
}

#[cfg(test)]
mod tests {
    use crate::structs::color::*;

    use super::Rgba;

    fn color_equal(color_a: Rgba<u8>, color_b: Rgba<u8>) -> bool {
        color_a.0[0] == color_b.0[0] &&
        color_a.0[1] == color_b.0[1] &&
        color_a.0[2] == color_b.0[2] &&
        color_a.0[3] == color_b.0[3]
    }

    #[test]
    fn test_color_scale() {
        assert!(
            color_equal(
                color_scale(&Rgba([16, 16, 16, 255]), 2.0), 
                Rgba([32, 32, 32, 255])
            )
        );

        assert!(
            color_equal(
                color_scale(&Rgba([32, 32, 32, 255]), 0.5), 
                Rgba([16, 16, 16, 255])
            )
        );
    }
}