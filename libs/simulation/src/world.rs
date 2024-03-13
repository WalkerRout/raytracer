
use lib_raytracer::prelude::*;

use std::fs::File;
use std::io::Write;

#[derive(Debug, Default)]
pub struct PpmWorld {
  bytes: Vec<u8>,
}

impl PpmWorld {
  pub fn new() -> Self {
    Self { bytes: Vec::new(), }
  }

  pub fn save_to_path<A: AsRef<str>>(self, path: A) -> Result<(), RaytracerError> {
    let mut file = File::create(path.as_ref())?;
    file.write_all(&self.bytes)?;
    Ok(())
  }
}

impl Scene for PpmWorld {
  fn render(&mut self, camera: &mut Camera, hittable: &mut impl Hittable) -> Result<usize, RaytracerError> {
    let Camera { config, .. } = *camera;
    let Config { image_width, image_height, .. } = config;
    let max_depth = 50;

    let mut rng = rand::thread_rng();

    writeln!(&mut self.bytes, "P3\n{} {}\n255", image_width, image_height)?;
    for j in 0..image_height {
      for i in 0..image_width {
        let mut pixel_colour = Colour::default();
        for _ in 0..camera.config.samples_per_pixel {
          let ray = camera.get_ray(&mut rng, i, j);
          pixel_colour += camera.ray_colour(&mut rng, &ray, hittable, max_depth);
        }
        write_colour(&mut self.bytes, &pixel_colour, camera.config.samples_per_pixel)?;
      }
    }

    Ok(self.bytes.len())
  }
}