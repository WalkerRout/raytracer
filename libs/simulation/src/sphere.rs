
use lib_raytracer::prelude::*;

use std::rc::Rc;

#[derive(Clone)]
pub struct Sphere {
  radius: f64,
  center: Point3,
  material: Rc<dyn Material>,
}

impl Sphere {
  pub fn new(radius: f64, center: Point3, material: Rc<dyn Material>) -> Self {
    Self { radius, center, material, }
  }
}

impl Hittable for Sphere {
  fn hit(&mut self, ray: &Ray, ray_i: Interval, record: &mut HitRecord) -> bool {
    let oc = ray.position() - self.center;
    let a = ray.direction().length_squared();
    let half_b = dot(oc, ray.direction());
    let c = oc.length_squared() - self.radius*self.radius;

    let discriminant = half_b*half_b - a*c;
    if discriminant < 0.0 {
      return false;
    }
    let sqrtd = discriminant.sqrt();

    let mut root = (-half_b - sqrtd) / a;
    if !ray_i.surrounds(root) {
      root = (-half_b + sqrtd) / a;
      if !ray_i.surrounds(root) {
        return false;
      }
    }

    record.d = root;
    record.position = ray.at(record.d);
    let outward_normal = (record.position - self.center) / self.radius;
    record.set_face_normal(ray, outward_normal);
    record.material = Some(Rc::clone(&self.material));

    true
  }
}