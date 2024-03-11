
use crate::*;

pub type Colour = vector::Vector3;

pub fn write_colour(colour: &Colour) {
  let r = colour.x.clamp(0.0, 1.0) * 255.0;
  let g = colour.y.clamp(0.0, 1.0) * 255.0;
  let b = colour.z.clamp(0.0, 1.0) * 255.0;
  println!("{r} {g} {b}");
}

#[cfg(test)]
mod tests {
  use super::*;
  use rstest::*;
}