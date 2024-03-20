
use simulation::*;

use ppm::Ppm;
use png::Png;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  let scene_option = "png";

  match scene_option {
    "ppm" => {
      let path = "./output.ppm";
      let mut ppm = Ppm::new();
      println!("Rendering scene at {path}...");
      let count = render_scene_with_world(&mut ppm)?;

      println!("Image rendered with {count} bytes, attempting to save...");
      ppm.save_to_path(path)?;
      println!("Scene saved... DONE");
    },
    "png" => {
      let path = "./output.png";
      let mut png = Png::new();
      println!("Rendering scene at {path}...");
      let count = render_scene_with_world(&mut png)?;

      println!("Image rendered with {count} bytes, attempting to save...");
      png.save_to_path(path)?;
      println!("Scene saved... DONE");
    },
    _ => ()
  }

  Ok(())
}
