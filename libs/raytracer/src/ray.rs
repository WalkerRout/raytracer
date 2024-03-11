
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