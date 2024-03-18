
use crate::*;

use interval::Interval;

use std::io::Write;

pub type Colour = vector::Vector3;

pub fn write_colour(f: &mut impl Write, colour: &Colour, samples_per_pixel: usize) -> Result<(), RaytracerError> {
  let interval = Interval::new(0.0, 1.0);
  let scale = 1.0 / samples_per_pixel as f64;
  let scaled_x = linear_to_gamma(colour.x * scale);
  let scaled_y = linear_to_gamma(colour.y * scale);
  let scaled_z = linear_to_gamma(colour.z * scale);
  let r = (interval.clamp(scaled_x) * 255.0) as u8;
  let g = (interval.clamp(scaled_y) * 255.0) as u8;
  let b = (interval.clamp(scaled_z) * 255.0) as u8;
  writeln!(f, "{r} {g} {b}")?;
  Ok(())
}

fn linear_to_gamma(lc: f64) -> f64 {
  lc.sqrt()
}

#[cfg(test)]
mod tests {
  use super::*;
  use rstest::*;

  mod colour {
    use super::*;

    #[rstest]
    #[case((0.0, 0.0, 0.0), "0 0 0\n")]
    #[case((1.0, 1.0, 1.0), "255 255 255\n")]
    #[case((1.0, 1.0, 0.0), "255 255 0\n")]
    #[case((5.0, 5.0, 5.0), "255 255 255\n")]
    fn write_colour(#[case] tuple: (f64, f64, f64), #[case] expected: &str) {
      struct DemoWriter(String);
      impl std::io::Write for DemoWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
          self.0 += &String::from_utf8(buf.to_vec()).unwrap();
          Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> { Ok(()) }
      }

      let mut dwriter = DemoWriter(String::new());
      let colour = Colour::from(tuple);
      assert!(super::write_colour(&mut dwriter, &colour, 1).is_ok());
      assert_eq!(dwriter.0, expected);
    }
  }
}