
use crate::*;

use config::Config;

pub trait Scene {
  fn config(&self) -> Config;
  fn save_to_path<A: AsRef<str>>(self, path: A) -> Result<(), RaytracerError>;
}
