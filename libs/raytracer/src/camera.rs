
#[allow(unused_imports)]
use crate::*;

use ray::{Ray, Hittable, HitRecord};
use point::Point3;
use colour::Colour;
use vector::Vector3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera {
  pub first_pixel: Point3,
  pub pixel_dx: Vector3,
  pub pixel_dy: Vector3,
  pub center: Point3,
  //pub config: Config,
}

impl Camera {
  pub fn new(
    focal_length: f64,
    center: Point3,
    (image_width, image_height): (usize, usize),
    (viewport_width, viewport_height): (f64, f64),
  ) -> Self {
    let viewport_x = Vector3::new(viewport_width, 0.0, 0.0);
    let viewport_y = Vector3::new(0.0, -viewport_height, 0.0);
    let pixel_dx = viewport_x / image_width as f64;
    let pixel_dy = viewport_y / image_height as f64;
    let viewport_upper_left = center - Vector3::new(0.0, 0.0, focal_length) - viewport_x/2.0 - viewport_y/2.0;
    let first_pixel = viewport_upper_left + 0.5 * (pixel_dx + pixel_dy);

    Self {
      first_pixel,
      pixel_dx,
      pixel_dy,
      center,
    }
  }

  pub fn ray_colour(&self, ray: &impl Ray, hittable: &mut impl Hittable) -> Colour {
    let mut record = HitRecord::default();
    if hittable.hit(ray, (0.0, f64::INFINITY), &mut record) {
      return 0.5 * (record.normal + Colour::new(1.0, 1.0, 1.0));
    }

    let unit_direction = ray.direction().into_unit();
    let a = 0.5*(unit_direction.y + 1.0);
    (1.0-a)*Colour::new(1.0, 1.0, 1.0) + a*Colour::new(0.5, 0.7, 1.0)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use rstest::*;
}