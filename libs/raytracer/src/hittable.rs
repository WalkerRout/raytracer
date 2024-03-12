
use crate::*;

use ray::Ray;
use point::Point3;
use vector::Vector3;
use interval::Interval;

#[derive(Debug, Default, Clone, Copy)]
pub struct HitRecord {
  pub position: Point3,
  pub normal: Vector3,
  pub d: f64,
  pub front_face: bool,
}

impl HitRecord {
  pub fn new(position: Point3, normal: Vector3, d: f64) -> Self {
    Self {
      position,
      normal,
      d,
      front_face: false,
    }
  }

  pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vector3) {
    self.front_face = vector::dot(ray.direction(), outward_normal) < 0.0;
    self.normal = if self.front_face { outward_normal } else { -outward_normal };
  }
}

pub trait Hittable {
  fn hit(&mut self, ray: &Ray, ray_i: Interval, record: &mut HitRecord) -> bool;
}


/// Default type for Vec of hittables
pub type VecOfHittable = Vec<Box<dyn Hittable>>;

impl Hittable for VecOfHittable {
  fn hit(&mut self, ray: &Ray, ray_i: Interval, record: &mut HitRecord) -> bool {
    let mut temp_record = HitRecord::default();
    let mut hit_anything = false;
    let mut closest_so_far = ray_i.max;

    for object in self {
      if object.hit(ray, Interval::new(ray_i.min, closest_so_far), &mut temp_record) {
        hit_anything = true;
        closest_so_far = temp_record.d;
        *record = temp_record;
      }
    }

    hit_anything
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use rstest::*;

  mod hit_record {
    use super::*;

    #[rstest]
    fn new() {
      let expected_d = 1.2;
      let expected_normal = Vector3::new(0.0, 1.0, 0.0);
      let expected_position = Point3::new(0.0, 0.0, 0.0);
      let record = HitRecord::new(expected_position, expected_normal, expected_d);
      assert_eq!(record.d, expected_d);
      assert_eq!(record.normal, expected_normal);
      assert_eq!(record.position, expected_position);
    }

    #[rstest]
    #[case(Vector3::new(0.0, 0.0, 0.0), false)]
    #[case(Vector3::new(3.0, 0.0, 0.0), false)]
    #[case(Vector3::new(-3.0, 0.0, 0.0), true)]
    #[case(Vector3::new(0.0, 10.0, 0.0), false)]
    #[case(Vector3::new(3.0, -5.0, 0.0), true)]
    #[case(Vector3::new(-2.0, -1.0, 0.0), true)]
    #[case(Vector3::new(-0.5, 4.0, 0.2), false)]
    fn set_face_normal(#[case] outward_normal: Vector3, #[case] expected: bool) { 
      let mut record = HitRecord::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0), 0.0);
      let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 0.0));
      record.set_face_normal(&ray, outward_normal);
      assert_eq!(record.front_face, expected);
    }
  }
}