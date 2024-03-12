
use lib_raytracer::prelude::*;

use std::fs::File;
use std::io::Write;

pub struct World {
  pub config: Config,
  pub objects: Vec<Box<dyn Hittable>>,
}

impl World {
  pub fn new(config: Config) -> Self {
    Self { 
      config,
      objects: vec![],
    }
  }
}

impl Scene for World {
  fn config(&self) -> Config {
    self.config
  }

  fn save_to_path<A: AsRef<str>>(mut self, path: A) -> Result<(), RaytracerError> {
    let mut destination = Vec::new();
    let Config { image_width, image_height, camera, .. } = self.config();
    let Camera { first_pixel, pixel_dx, pixel_dy, center, } = camera;

    writeln!(&mut destination, "P3\n{} {}\n255", image_width, image_height)?;
    for j in 0..image_height {
      for i in 0..image_width {
        let pixel_center = first_pixel + (i as f64 * pixel_dx) + (j as f64 * pixel_dy);
        let ray_direction = pixel_center - center;
        let ray = StandardRay::new(center, ray_direction);
        let pixel_colour = camera.ray_colour(&ray, &mut self);
        write_colour(&mut destination, &pixel_colour)?;
      }
    }

    let mut file = File::create(path.as_ref())?;
    file.write_all(&destination)?;

    Ok(())
  }
}

impl Hittable for World {
  fn hit(&mut self, ray: &dyn Ray, (ray_min, ray_max): (f64, f64), record: &mut HitRecord) -> bool {
    let mut temp_record = HitRecord::default();
    let mut hit_anything = false;
    let mut closest_so_far = ray_max;

    for object in &mut self.objects {
      if object.hit(ray, (ray_min, closest_so_far), &mut temp_record) {
        hit_anything = true;
        closest_so_far = temp_record.t;
        *record = temp_record;
      }
    }

    hit_anything
  }
}

