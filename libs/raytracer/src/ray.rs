
use crate::*;

use point::Point3;
use vector::Vector3;

pub trait Ray {
  fn position(&self) -> Point3;
  fn direction(&self) -> Vector3;

  fn at(&self, d: f64) -> Point3 {
    self.position() + d * self.direction()
  }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct StandardRay {
  position: Point3,
  direction: Vector3,
}

impl StandardRay {
  pub fn new(position: Point3, direction: Vector3) -> Self {
    Self { position, direction, }
  }
}

impl Ray for StandardRay {
  fn position(&self) -> Point3 {
    self.position
  }

  fn direction(&self) -> Vector3 {
    self.direction
  }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct HitRecord {
  pub position: Point3,
  pub normal: Vector3,
  pub t: f64,
  pub front_face: bool,
}

impl HitRecord {
  pub fn new(position: Point3, normal: Vector3, t: f64) -> Self {
    Self {
      position,
      normal,
      t,
      front_face: false,
    }
  }

  pub fn set_face_normal(&mut self, ray: &dyn Ray, outward_normal: Vector3) {
    self.front_face = vector::dot(ray.direction(), outward_normal) < 0.0;
    self.normal = if self.front_face { outward_normal } else { -outward_normal };
  }
}

pub trait Hittable {
  fn hit(&mut self, ray: &dyn Ray, ray_min_max: (f64, f64), record: &mut HitRecord) -> bool;
}

#[cfg(test)]
mod tests {
  use super::*;
  use rstest::*;

  mod standard_ray {
    use super::*;

    #[rstest]
    fn new() {
      let position = Point3::new(0.0, 0.0, 1.0);
      let direction = Vector3::new(0.0, 1.0, 0.0);
      let ray = StandardRay::new(position, direction);
      assert_eq!(ray.position, position);
      assert_eq!(ray.direction, direction);
    }

    #[rstest]
    #[case((0.0, 0.0, 0.0))]
    #[case((-1.0, 0.0, 3.0))]
    #[case((100.0, 100.0, 100.0))]
    fn position(#[case] tuple: (f64, f64, f64)) {
      let ray = StandardRay::new(tuple.into(), Vector3::default());
      assert_eq!(ray.position(), tuple.into());
    }

    #[rstest]
    #[case((0.0, 0.0, 0.0))]
    #[case((-1.0, 0.0, 3.0))]
    #[case((100.0, 100.0, 100.0))]
    fn direction(#[case] tuple: (f64, f64, f64)) {
      let ray = StandardRay::new(Point3::default(), tuple.into());
      assert_eq!(ray.direction(), tuple.into());
    }

    #[rstest]
    #[case(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0),   0.0, Point3::new(0.0, 0.0, 0.0))]
    #[case(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0),   1.0, Point3::new(0.0, 1.0, 0.0))]
    #[case(Point3::new(5.0, 5.0, 0.0), Vector3::new(10.0, 0.0, 0.0),  0.5, Point3::new(10.0, 5.0, 0.0))]
    #[case(Point3::new(5.0, 5.0, 0.0), Vector3::new(10.0, 0.0, 0.0), -0.5, Point3::new(0.0, 5.0, 0.0))]
    fn at(
      #[case] position: Point3, 
      #[case] direction: Vector3, 
      #[case] d: f64, 
      #[case] expected: Point3
    ) {
      let ray = StandardRay::new(position, direction);
      assert_eq!(ray.at(d), expected);
    }
  }
}