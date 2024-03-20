
use lib_raytracer::prelude::*;

pub mod ppm;
pub mod png;
pub mod sphere;

use sphere::Sphere;

use std::sync::Arc;

pub fn generate_world() -> (Camera, impl Hittable) {
  let width = 1000;
  let aspect_ratio = 16.0 / 9.0;

  let camera = Camera::new(width, aspect_ratio);
  let objects: VecOfHittable = {
    let material_ground = Arc::new(Lambertian::new(Colour::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Colour::new(0.7, 0.3, 0.3)));
    let material_left   = Arc::new(Dielectric::new(1.5));
    let material_right  = Arc::new(Metal::new(Colour::new(0.8, 0.6, 0.2), 0.0));

    vec![
      Box::new(Sphere::new(100.0, Point3::new(0.0, -100.5, -1.0), material_ground)),
      Box::new(Sphere::new(0.5, Point3::new(0.0, 0.0, -1.0), material_center)),
      Box::new(Sphere::new(0.5, Point3::new(-1.0, 0.0, -1.0), material_left)),
      Box::new(Sphere::new(0.5, Point3::new(1.0, 0.0, -1.0), material_right)),
    ]
  };

  (camera, objects)
}

pub fn render_scene_with_world(scene: &mut impl Scene) -> Result<usize, RaytracerError> {
  let (mut camera, mut hittable) = generate_world();
  scene.render(&mut camera, &mut hittable)
}