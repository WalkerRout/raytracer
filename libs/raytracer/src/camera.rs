
use rand::Rng;

#[allow(unused_imports)]
use crate::*;

use ray::Ray;
use point::Point3;
use colour::Colour;
use vector::Vector3;
use interval::Interval;
use hittable::{Hittable, HitRecord};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Config {
  pub aspect_ratio: f64,
  pub image_width: usize,
  pub image_height: usize,
  pub viewport_width: f64,
  pub viewport_height: f64,
  pub focal_length: f64,
  pub first_pixel: Point3,
  pub pixel_dx: Vector3,
  pub pixel_dy: Vector3,
  pub samples_per_pixel: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera {
  pub center: Point3,
  pub config: Config,
}

impl Camera {
  pub fn new(image_width: usize, aspect_ratio: f64) -> Self {
    let focal_length = 1.0;
    let samples_per_pixel = 100;

    let image_height = (image_width as f64 / aspect_ratio).max(1.0) as usize;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let center = Point3::new(0.0, 0.0, 0.0);
    let viewport_x = Vector3::new(viewport_width, 0.0, 0.0);
    let viewport_y = Vector3::new(0.0, -viewport_height, 0.0);
    let pixel_dx = viewport_x / image_width as f64;
    let pixel_dy = viewport_y / image_height as f64;
    let viewport_upper_left = center - Vector3::new(0.0, 0.0, focal_length) - viewport_x/2.0 - viewport_y/2.0;
    let first_pixel = viewport_upper_left + 0.5 * (pixel_dx + pixel_dy);

    let config = Config {
      aspect_ratio,
      image_width,
      image_height,
      viewport_width,
      viewport_height,
      focal_length,
      first_pixel,
      pixel_dx,
      pixel_dy,
      samples_per_pixel,
    };

    Self {
      center,
      config,
    }
  }

  pub fn get_ray(&self, rng: &mut impl Rng, i: usize, j: usize) -> Ray {
    let Config { first_pixel, pixel_dx, pixel_dy, .. } = self.config;
    let pixel_center = first_pixel + (i as f64 * pixel_dx) + (j as f64 * pixel_dy);
    let pixel_sample = pixel_center + self.pixel_sample_square(rng);
    let ray_origin = self.center;
    let ray_direction = pixel_sample - ray_origin;
    Ray::new(ray_origin, ray_direction)
  }

  pub fn ray_colour(&self, rng: &mut impl Rng, ray: &Ray, hittable: &mut impl Hittable, depth: usize) -> Colour {
    let mut record = HitRecord::default();
    if depth == 0 {
      Colour::new(0.0, 0.0, 0.0)
    } else if hittable.hit(ray, Interval::new(0.001, f64::INFINITY), &mut record) {
      let direction = record.normal + Vector3::random_unit_vector(rng);
      0.5 * self.ray_colour(rng, &Ray::new(record.position, direction), hittable, depth-1)
    } else {
      let unit_direction = ray.direction().into_unit();
      let a = 0.5*(unit_direction.y + 1.0);
      (1.0-a)*Colour::new(1.0, 1.0, 1.0) + a*Colour::new(0.5, 0.7, 1.0)
    }
  }

  pub fn pixel_sample_square(&self, rng: &mut impl Rng) -> Point3 {
    let px = -0.5 + rng.gen::<f64>();
    let py = -0.5 + rng.gen::<f64>();
    px * self.config.pixel_dx + py * self.config.pixel_dy
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use rstest::*;

  mod camera {
    use super::*;

    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[rstest]
    #[case(10, 16.0/9.0, Vector3::new(0.4, 0.0, 0.0), Vector3::new(0.0, -0.4, 0.0), Point3::new(-1.8, 0.8, -1.0))]
    #[case(500, 2.0, Vector3::new(0.008, 0.0, 0.0), Vector3::new(0.0, -0.008, 0.0), Point3::new(-1.996, 0.996, -1.0))]
    #[case(500, 1.0, Vector3::new(0.004, 0.0, 0.0), Vector3::new(0.0, -0.004, 0.0), Point3::new(-0.998, 0.998, -1.0))]
    #[case(500, 0.5, Vector3::new(0.002, 0.0, 0.0), Vector3::new(0.0, -0.002, 0.0), Point3::new(-0.499, 0.999, -1.0))]
    #[should_panic]
    #[case(0, 16.0/9.0, Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 0.0))]
    fn new(
      #[case] image_width: usize,
      #[case] aspect_ratio: f64,
      #[case] expected_pixel_dx: Vector3,
      #[case] expected_pixel_dy: Vector3,
      #[case] expected_first_pixel: Point3,
    ) {
      let camera = Camera::new(image_width, aspect_ratio);
      assert_eq!(camera.config.pixel_dx, expected_pixel_dx);
      assert_eq!(camera.config.pixel_dy, expected_pixel_dy);
      assert_eq!(camera.config.first_pixel, expected_first_pixel);
    }

    #[rstest]
    #[case((0, 0), Ray::new(Point3::default(), Vector3::new(-1.9454483046154663, 0.9239779673862012, -1.0)))]
    #[case((49, 24), Ray::new(Point3::default(), Vector3::new(1.9745516953845337, -0.9960220326137987, -1.0)))]
    #[case((10, 11), Ray::new(Point3::default(), Vector3::new(-1.1454483046154662, 0.043977967386201244, -1.0)))]
    #[should_panic]
    #[case((50, 0), Ray::new(Point3::default(), Vector3::default()))]
    #[should_panic]
    #[case((0, 25), Ray::new(Point3::default(), Vector3::default()))]
    #[should_panic]
    #[case((50, 25), Ray::new(Point3::default(), Vector3::default()))]
    fn get_ray(#[case] indices: (usize, usize), #[case] expected: Ray) {
      let mut rng = ChaCha8Rng::seed_from_u64(42);
      let camera = Camera::new(50, 2.0);
      assert_eq!(camera.get_ray(&mut rng, indices.0, indices.1), expected);
    }

    #[rstest]
    fn ray_colour_toggle() { 
      struct Toggle(bool);
      impl Hittable for Toggle {
        fn hit(&mut self, _: &Ray, _: Interval, record: &mut HitRecord) -> bool {
          // ensure HitRecord stays as default!
          *record = HitRecord::default();
          // Toggle .0
          let res = self.0;
          self.0 = !self.0;
          res
        }
      }

      let mut rng = ChaCha8Rng::seed_from_u64(4);
      let camera = Camera::new(500, 16.0 / 9.0);
      let ray = Ray::default();

      let mut toggle = Toggle(true);
      for _ in 0..100 {
        let expected_background = {
          let unit_direction = ray.direction().into_unit();
          let a = 0.5*(unit_direction.y + 1.0);
          (1.0-a)*Colour::new(1.0, 1.0, 1.0) + a*Colour::new(0.5, 0.7, 1.0)
        };
        let colour = camera.ray_colour(&mut rng, &ray, &mut toggle, 10);
        if toggle.0 {
          assert_ne!(colour, expected_background);
        } else {
          assert_eq!(colour, expected_background);
        }
      }
    }

    #[rstest]
    fn pixel_sample_square() {
      let mut rng = ChaCha8Rng::seed_from_u64(4);
      let camera = Camera::new(50, 2.0);
      let expected = vec![
        Point3::new(0.01810289656719271, 0.03441457162334653, 0.0),
        Point3::new(0.030279376124643455, -0.02092136846687394, 0.0),
        Point3::new(-0.024847111286775157, -0.029056899399724348, 0.0),
        Point3::new(-0.03411305294726887, 0.0015556901526561973, 0.0),
      ];
      for expected in expected {
        assert_eq!(camera.pixel_sample_square(&mut rng), expected);
      }
    }
  }
}