use std::cmp::min;
use raster::Color;

pub fn color_add(lhs: &Color, rhs: &Color) -> Color {
    // default to full-alpha, clamp rgb vals
    let new_r = min((lhs.r as u64) + (rhs.r as u64), 0xFF) as u8;
    let new_g = min((lhs.g as u64) + (rhs.g as u64), 0xFF) as u8;
    let new_b = min((lhs.b as u64) + (rhs.b as u64), 0xFF) as u8;

    Color { 
        r: new_r, 
        g: new_g, 
        b: new_b, 
        a: 0xFF 
    }
}

pub fn color_multiply(lhs: &Color, rhs: &Color) -> Color {
    let new_r = ((lhs.r as u64) * (rhs.r as u64) / 0xFF) as u8;
    let new_g = ((lhs.g as u64) * (rhs.g as u64) / 0xFF) as u8;
    let new_b = ((lhs.b as u64) * (rhs.b as u64) / 0xFF) as u8;

    Color { 
        r: new_r, 
        g: new_g, 
        b: new_b, 
        a: 0xFF 
    }
}

pub fn color_scale<T>(color: &Color, factor: T) -> Color 
    where T: Into<f64> + Copy
{
    let new_r = (color.r as f64 * factor.into()) as u8;
    let new_g = (color.g as f64 * factor.into()) as u8;
    let new_b = (color.b as f64 * factor.into()) as u8;

    Color {
        r: new_r,
        g: new_g,
        b: new_b,
        a: 0xFF
    }
}

#[cfg(test)]
mod tests {
    use crate::structs::color::*;

    use super::Color;

    fn color_equal(color_a: Color, color_b: Color) -> bool {
        color_a.r == color_b.r &&
        color_a.g == color_b.g &&
        color_a.b == color_b.b &&
        color_a.a == color_b.a
    }

    #[test]
    fn test_color_scale() {
        assert!(
            color_equal(
                color_scale(&Color::hex("#111111").unwrap(), 2.0), 
                Color::hex("#222222").unwrap()
            )
        );

        assert!(
            color_equal(
                color_scale(&Color::hex("#222222").unwrap(), 0.5), 
                Color::hex("#111111").unwrap()
            )
        );
    }
}