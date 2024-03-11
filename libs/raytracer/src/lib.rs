
pub mod ray;
pub mod point;
pub mod colour;
pub mod vector;

pub mod prelude {
  pub use super::{
    ray::*,
    point::*,
    colour::*,
    vector::*,
  };
}