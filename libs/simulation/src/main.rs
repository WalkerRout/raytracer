
use std::error::Error;

use lib_raytracer::prelude::*;

mod world;
mod sphere;

use world::PpmWorld;
use sphere::Sphere;

fn main() -> Result<(), Box<dyn Error>> {
  let path = "./output.ppm";
  let width = 4000;
  let aspect_ratio = 16.0 / 9.0;

  let mut camera = Camera::new(width, aspect_ratio);
  let mut objects = {
    vec![
      Box::new(Sphere::new(0.5, Point3::new(0.0, 0.0, -1.0))) as Box<dyn Hittable>,
      Box::new(Sphere::new(100.0, Point3::new(0.0, -100.5, -1.0))) as Box<dyn Hittable>,
    ]
  };

  // PpmWorld
  {
    let mut ppm = PpmWorld::new();
    println!("{}x{} image generating at {}...", 
    camera.config.image_width, camera.config.image_height, path);

    let count = ppm.render(&mut camera, &mut objects)?;
    println!("Image rendered with {count} bytes, attempting to save...");
    ppm.save_to_path(path)?;
    println!("Scene saved... DONE");
  }

  Ok(())
}
