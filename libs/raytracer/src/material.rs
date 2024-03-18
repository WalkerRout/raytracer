
use crate::*;

use ray::Ray;
use colour::Colour;
use vector::Vector3;
use hittable::HitRecord;

pub trait Material {
  /// Produces whether the ray scatters
  fn scatter(
    &self,
    in_ray: &Ray, 
    record: &HitRecord, 
    attenuation: &mut Colour, 
    scattered: &mut Ray
  ) -> bool;
}

pub struct Lambertian {
  pub albedo: Colour,
}

impl Lambertian {
  pub fn new(albedo: Colour) -> Self {
    Self { albedo, }
  }
}

impl Material for Lambertian {
  fn scatter(
    &self,
    _: &Ray, 
    record: &HitRecord,
    attenuation: &mut Colour, 
    scattered: &mut Ray,
  ) -> bool {
    let mut rng = rand::thread_rng();
    let mut direction = record.normal + Vector3::random_unit_vector(&mut rng);
    if vector::near_zero(direction) {
      direction = record.normal;
    }
    *scattered = Ray::new(record.position, direction);
    *attenuation = self.albedo;
    true
  }
}

pub struct Metal {
  pub albedo: Colour,
}

impl Metal {
  pub fn new(albedo: Colour) -> Self {
    Self { albedo, }
  }
}

impl Material for Metal {
  fn scatter(
    &self,
    ray_in: &Ray, 
    record: &HitRecord,
    attenuation: &mut Colour, 
    scattered: &mut Ray,
  ) -> bool {
    let reflected = vector::reflect(ray_in.direction().to_unit(), record.normal);
    *scattered = Ray::new(record.position, reflected);
    *attenuation = self.albedo;
    true
  }
}

#[cfg(test)]
mod tests {
  #[allow(unused_imports)]
  use super::*;
  #[allow(unused_imports)]
  use rstest::*;

  mod lambertian {
    #[allow(unused_imports)]
    use super::*;
  }
}