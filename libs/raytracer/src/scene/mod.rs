
use crate::*;

pub mod base_scene;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Config {
  pub aspect_ratio: f64,
  pub image_width: usize,
  pub image_height: usize,
  pub viewport_width: f64,
  pub viewport_height: f64,
}

impl Default for Config {
  fn default() -> Self {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 500;
    let image_height = (image_width / aspect_ratio as usize).max(1);
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

    Self {
      aspect_ratio,
      image_width,
      image_height,
      viewport_width,
      viewport_height,
    }
  }
}

pub trait Scene {
  fn config(&self) -> Config;
  fn save_to_path<A: AsRef<str>>(self, path: A) -> Result<(), RaytracerError>;
}
