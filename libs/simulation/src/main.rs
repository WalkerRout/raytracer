
use lib_raytracer::prelude::*;

fn main() {
  let scene = BaseScene::default();
  match scene.save_to_path("output.ppm") {
    Ok(_) => println!("Scene saved"),
    Err(e) => println!("Error saving - {e}"),
  }
}
