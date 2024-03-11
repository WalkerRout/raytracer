
use crate::*;
use super::*; // scene::*;

use colour::Colour;

use std::fs::File;
use std::io::Write;

#[derive(Debug, Default)]
pub struct BaseScene {
  config: Config,
}

impl Scene for BaseScene {
  fn config(&self) -> Config {
    self.config
  }

  fn save_to_path<A: AsRef<str>>(self, path: A) -> Result<(), RaytracerError> {
    let mut destination = File::create(path.as_ref())?;
    let Config {
      image_width, 
      image_height,
      ..
    } = self.config();

    write!(&mut destination, "P3\n{} {}\n255", image_width, image_height)?;
    for j in 0..image_height {
      for i in 0..image_width {
        let colour = Colour::new(
          i as f64 / image_width as f64 - 1.0,
          j as f64 / image_height as f64 - 1.0,
          0.0,
        );
        colour::write_colour(&mut destination, &colour)?;
      }
    }
    Ok(())
  }
}