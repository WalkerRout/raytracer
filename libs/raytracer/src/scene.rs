
use crate::*;

use camera::Camera;
use hittable::Hittable;

pub trait Scene/*: std::io::Write*/ {
  /// Produce number of bytes written
  fn render(&mut self, camera: &mut Camera, hittable: &mut impl Hittable) -> Result<usize, RaytracerError>;
}
