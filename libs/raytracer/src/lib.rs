
use thiserror::Error;

use std::io;

pub mod ray;
pub mod point;
pub mod scene;
pub mod config;
pub mod camera;
pub mod colour;
pub mod vector;

pub mod prelude {
  #[allow(unused_imports)]
  pub use super::{
    ray::*,
    point::*,
    scene::*,
    config::*,
    camera::*,
    colour::*,
    vector::*,
    RaytracerError,
  };
}

#[derive(Debug, Error)]
pub enum RaytracerError {
  #[error("IO error during raytrace - {source}")]
  Io {
    #[from]
    source: io::Error,
  },
}