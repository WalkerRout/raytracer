
use lib_raytracer::prelude::*;

use image::{Rgba, DynamicImage};

use std::path::Path;

#[derive(Debug, Default)]
pub struct Png {
  image: Option<DynamicImage>,
}

impl Png {
  pub fn new() -> Self {
    Self { image: None, }
  }

  pub fn save_to_path<P: AsRef<Path>>(self, path: P) -> Result<(), RaytracerError> {
    if let Some(image) = self.image {
      image.save(path).map_err(|_| RaytracerError::SceneSaveError)
    } else {
      Err(RaytracerError::SceneSaveError)
    }
  }
}

impl Scene for Png {
  fn render(&mut self, camera: &mut Camera, hittable: &mut impl Hittable) -> Result<usize, RaytracerError> {
    let Camera { config, .. } = *camera;
    let Config { image_width, image_height, .. } = config;
    let max_depth = 50;
    let mut rng = rand::thread_rng();

    self.image = Some(DynamicImage::new_rgba8(image_width as u32, image_height as u32));
    let image = self.image.as_mut().unwrap();

    image
      .as_mut_rgba8().unwrap() // safe; we create as rgba8
      .enumerate_pixels_mut()
      .for_each(|(i, j, pixel)| {
        let pixel_colour = (0..camera.config.samples_per_pixel)
          .fold(Colour::default(), |pixel_colour, _| {
            let ray = camera.get_ray(&mut rng, i as usize, j as usize);
            pixel_colour + camera.ray_colour(&mut rng, &ray, hittable, max_depth)
          });
        let pixel_colour = colour_to_pixel(&pixel_colour, camera.config.samples_per_pixel);
        *pixel = Rgba([pixel_colour.r, pixel_colour.g, pixel_colour.b, 255]);
      });

    Ok(self.image.as_ref().unwrap().as_bytes().len())
  }
}