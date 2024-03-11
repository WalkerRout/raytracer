
use crate::*;

use std::io::Write;

pub type Colour = vector::Vector3;

pub fn write_colour(f: &mut impl Write, colour: &Colour) -> Result<(), RaytracerError> {
  let r = (colour.x.clamp(0.0, 1.0) * 255.0) as u8;
  let g = (colour.y.clamp(0.0, 1.0) * 255.0) as u8;
  let b = (colour.z.clamp(0.0, 1.0) * 255.0) as u8;
  write!(f, "{r} {g} {b}")?;
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use rstest::*;
}