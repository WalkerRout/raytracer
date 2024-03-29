
use thiserror::Error;

use std::io;

pub mod ray;
pub mod point;
pub mod scene;
pub mod camera;
pub mod colour;
pub mod vector;
pub mod interval;
pub mod material;
pub mod hittable;

pub mod prelude {
  #[allow(unused_imports)]
  pub use super::{
    ray::*,
    point::*,
    scene::*,
    camera::*,
    colour::*,
    vector::*,
    interval::*,
    material::*,
    hittable::*,
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
  #[error("unable to render scene")]
  SceneRenderError,
  #[error("unable to save scene")]
  SceneSaveError,
}