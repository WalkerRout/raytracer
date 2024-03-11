
use lib_raytracer::prelude::*;

fn main() {
  // Image
  let image_width = 256;
  let image_height = 256;

  // Render
  println!("P3\n{} {}\n255", image_width, image_height);

  for j in 0..image_height {
    for i in 0..image_width {
      let colour = Colour::new(
        f64::from(i) / f64::from(image_width - 1),
        f64::from(j) / f64::from(image_height - 1),
        0.0,
      );
      write_colour(&colour;)
    }
  }
}
