
use rand::Rng;

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
  pub fuzz_radius: f64,
}

impl Metal {
  pub fn new(albedo: Colour, fuzz_radius: f64) -> Self {
    Self { albedo, fuzz_radius, }
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
    let mut rng = rand::thread_rng();
    let reflected = vector::reflect(ray_in.direction().to_unit(), record.normal);
    let direction = reflected + self.fuzz_radius*Vector3::random_unit_vector(&mut rng);
    
    *scattered = Ray::new(record.position, direction);
    *attenuation = self.albedo;

    vector::dot(scattered.direction(), record.normal) > 0.0
  }
}

pub struct Dielectric {
  pub refraction_index: f64,
}

impl Dielectric {
  pub fn new(refraction_index: f64) -> Self {
    Self { refraction_index, }
  }
}

impl Material for Dielectric {
  fn scatter(
    &self,
    ray_in: &Ray, 
    record: &HitRecord,
    attenuation: &mut Colour, 
    scattered: &mut Ray,
  ) -> bool {
    let schlick_reflectance = |cos: f64, ref_index: f64| -> f64 {
      let r0 = (1.0-ref_index) / (1.0+ref_index);
      let r0 = r0 * r0;
      r0 + (1.0-r0)*(1.0-cos).powi(5)
    };
    let mut rng = rand::thread_rng();
    let refraction_ratio = if record.front_face { 
      1.0/self.refraction_index 
    } else { 
      self.refraction_index 
    };
    let unit_direction = ray_in.direction().to_unit();    
    let cos_theta = vector::dot(-unit_direction, record.normal).min(1.0);
    let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();
    
    let no_solutions = refraction_ratio * sin_theta > 1.0;
    let direction = if no_solutions || schlick_reflectance(cos_theta, refraction_ratio) > rng.gen::<f64>() {
      // reflect
      vector::reflect(unit_direction, record.normal)
    } else {
      // otherwise refract
      vector::refract(unit_direction, record.normal, refraction_ratio)
    };


    *attenuation = Colour::new(1.0, 1.0, 1.0);
    *scattered = Ray::new(record.position, direction);
    
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

    #[rstest]
    fn new() {
      todo!()
    }

    #[rstest]
    fn scatter() {
      todo!()
    }
  }

  mod metal {
    #[allow(unused_imports)]
    use super::*;

    #[rstest]
    fn new() {
      todo!()
    }

    #[rstest]
    fn scatter() {
      todo!()
    }
  }

  mod dielectric {
    #[allow(unused_imports)]
    use super::*;

    #[rstest]
    fn new() {
      todo!()
    }

    #[rstest]
    fn scatter() {
      todo!()
    }
  }
}