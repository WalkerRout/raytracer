
use lib_raytracer::prelude::*;

mod scene;
mod sphere;

use scene::World;
use sphere::Sphere;

fn main() {
  let path = "output.ppm";
  let width = 500;
  let aspect_ratio = 16.0 / 9.0;
  let mut scene = World::new(Config::new(width, aspect_ratio));
  scene.objects.push(Box::new(Sphere::new(0.5, Point3::new(0.0, 0.0, -1.0))));
  scene.objects.push(Box::new(Sphere::new(100.0, Point3::new(0.0, -100.5, -1.0))));

  println!("{}x{} image generating at {}...", 
    scene.config.image_width, scene.config.image_height, path);

  match scene.save_to_path(path) {
    Ok(_) => println!("Scene saved"),
    Err(e) => println!("Error saving - {e}"),
  }
}
