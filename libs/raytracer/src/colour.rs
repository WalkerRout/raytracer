
use crate::*;

use interval::Interval;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Pixel {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}

impl Pixel {
  pub fn new(r: u8, g: u8, b: u8) -> Self {
    Self { r, g, b, }
  }
}

pub type Colour = vector::Vector3;

pub fn colour_to_pixel(colour: &Colour, samples_per_pixel: usize) -> Pixel {
  let interval = Interval::new(0.0, 1.0);
  let scale = 1.0 / samples_per_pixel as f64;
  let scaled_x = linear_to_gamma(colour.x * scale);
  let scaled_y = linear_to_gamma(colour.y * scale);
  let scaled_z = linear_to_gamma(colour.z * scale);

  let r = (interval.clamp(scaled_x) * 255.0) as u8;
  let g = (interval.clamp(scaled_y) * 255.0) as u8;
  let b = (interval.clamp(scaled_z) * 255.0) as u8;

  Pixel::new(r, g, b)
}

fn linear_to_gamma(lc: f64) -> f64 {
  lc.sqrt()
}

#[cfg(test)]
mod tests {
  use super::*;
  use rstest::*;

  mod pixel {
    use super::*;

    #[rstest]
    fn new() {
      let (r, g, b) = (0, 10, 255);
      let pixel = Pixel::new(r, g, b);
      assert_eq!(pixel.r, r);
      assert_eq!(pixel.g, g);
      assert_eq!(pixel.b, b);
    }
  }

  mod colour {
    use super::*;

    #[rstest]
    #[case((0.0, 0.0, 0.0), Pixel::new(0, 0, 0))]
    #[case((1.0, 1.0, 1.0), Pixel::new(255, 255, 255))]
    #[case((0.33, 0.4, 0.7), Pixel::new(146, 161, 213))]
    #[case((5.0, 5.0, 5.0), Pixel::new(255, 255, 255))]
    fn colour_to_pixel(#[case] tuple: (f64, f64, f64), #[case] expected: Pixel) {
      let colour = Colour::from(tuple);
      let pixel = super::colour_to_pixel(&colour, 1);
      assert_eq!(pixel, expected);
    }
  }
}