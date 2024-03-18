
use lib_raytracer::prelude::*;

use std::rc::Rc;
use std::error::Error;

mod world;
mod sphere;

use world::PpmWorld;
use sphere::Sphere;

fn main() -> Result<(), Box<dyn Error>> {
  let path = "./output.ppm";
  let width = 1000;
  let aspect_ratio = 16.0 / 9.0;

  let mut camera = Camera::new(width, aspect_ratio);
  let mut objects: VecOfHittable = {
    let material_ground = Rc::new(Lambertian::new(Colour::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Colour::new(0.7, 0.3, 0.3)));
    let material_left   = Rc::new(Metal::new(Colour::new(0.8, 0.8, 0.8), 0.4));
    let material_right  = Rc::new(Metal::new(Colour::new(0.8, 0.6, 0.2), 0.0));

    vec![
      Box::new(Sphere::new(100.0, Point3::new(0.0, -100.5, -1.0), material_ground)),
      Box::new(Sphere::new(0.5, Point3::new(0.0, 0.0, -1.0), material_center)),
      Box::new(Sphere::new(0.5, Point3::new(-1.0, 0.0, -1.0), material_left)),
      Box::new(Sphere::new(0.5, Point3::new(1.0, 0.0, -1.0), material_right)),
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
