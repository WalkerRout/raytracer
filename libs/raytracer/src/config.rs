
use crate::*;

use point::Point3;
use camera::Camera;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Config {
  pub camera: Camera,
  pub aspect_ratio: f64,
  pub image_width: usize,
  pub image_height: usize,
  pub viewport_width: f64,
  pub viewport_height: f64,
}

impl Config {
  pub fn new(image_width: usize, aspect_ratio: f64) -> Self {
    let image_height = (image_width as f64 / aspect_ratio).max(1.0) as usize;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera = Camera::new(
      1.0, Point3::new(0.0, 0.0, 0.0),
      (image_width, image_height), 
      (viewport_width, viewport_height)
    );

    Self {
      camera,
      aspect_ratio,
      image_width,
      image_height,
      viewport_width,
      viewport_height,
    }
  }
}