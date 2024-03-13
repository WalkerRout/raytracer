
use rand::Rng;

#[allow(unused_imports)]
use crate::*;

use std::fmt;
use std::ops::{Neg, Add, Sub, Mul, Div, Index, IndexMut, AddAssign, MulAssign, DivAssign};

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector3 {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl Vector3 {
  pub fn new(x: f64, y: f64, z: f64) -> Self {
    Self { x, y, z, }
  }

  pub fn random(rng: &mut impl Rng) -> Self {
    Self {
      x: rng.gen(),
      y: rng.gen(),
      z: rng.gen(),
    }
  }
  
  pub fn random_range(rng: &mut impl Rng, min: f64, max: f64) -> Self {
    Self {
      x: rng.gen_range(min..=max),
      y: rng.gen_range(min..=max),
      z: rng.gen_range(min..=max),
    }
  }

  pub fn random_in_unit_sphere(rng: &mut impl Rng) -> Self {
    loop {
      let p = Vector3::random_range(rng, -1.0, 1.0);
      if p.length_squared() < 1.0 {
        return p;
      }
    }
  }

  pub fn random_unit_vector(rng: &mut impl Rng) -> Self {
    let vec = Self::random_in_unit_sphere(rng);
    vec.into_unit()
  }

  pub fn random_on_hemisphere(rng: &mut impl Rng, normal: Self) -> Self {
    let on_unit_sphere = Self::random_unit_vector(rng);
    // in same hemisphere as normal
    if dot(on_unit_sphere, normal) > 0.0 {
      on_unit_sphere
    } else {
      -on_unit_sphere
    }
  }

  pub fn into_unit(self) -> Self {
    self / self.length()
  }

  pub fn length(&self) -> f64 {
    self.length_squared().sqrt()
  } 

  pub fn length_squared(&self) -> f64 {
    self.x*self.x + self.y*self.y + self.z*self.z
  }
}

impl From<(f64, f64, f64)> for Vector3 {
  fn from(fs: (f64, f64, f64)) -> Self {
    Self::new(fs.0, fs.1, fs.2)
  }
}

impl From<[f64; 3]> for Vector3 {
  fn from(fs: [f64; 3]) -> Self {
    Self::new(fs[0], fs[1], fs[2])
  }
}

impl Neg for Vector3 {
  type Output = Self;

  fn neg(self) -> Self::Output {
    Self::new(-self.x, -self.y, -self.z)
  }
}

impl Index<usize> for Vector3 {
  type Output = f64;

  fn index(&self, index: usize) -> &Self::Output {
    match index {
      0 => &self.x,
      1 => &self.y,
      2 => &self.z,
      _ => panic!("index out of bounds for Vector3"),
    }
  }
}

impl IndexMut<usize> for Vector3 {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    match index {
      0 => &mut self.x,
      1 => &mut self.y,
      2 => &mut self.z,
      _ => panic!("index out of bounds for Vector3"),
    }
  }
}

impl AddAssign for Vector3 {
  fn add_assign(&mut self, other: Self) {
    *self = Self::new(
      self.x + other.x,
      self.y + other.y,
      self.z + other.z,
    );
  }
}

impl MulAssign<f64> for Vector3 {
  fn mul_assign(&mut self, other: f64) {
    *self = Self::new(
      self.x * other,
      self.y * other,
      self.z * other,
    );
  }
}

impl DivAssign<f64> for Vector3 {
  fn div_assign(&mut self, other: f64) {
    *self *= 1.0/other;
  }
}

impl fmt::Display for Vector3 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "vec3({}, {}, {})", self.x, self.y, self.z)
  }
}

impl Add for Vector3 {
  type Output = Self;

  fn add(self, other: Self) -> Self::Output {
    Self::new(
      self.x + other.x,
      self.y + other.y,
      self.z + other.z,
    )
  }
}

impl Sub for Vector3 {
  type Output = Self;
  
  fn sub(self, other: Self) -> Self::Output {
    Self::new(
      self.x - other.x,
      self.y - other.y,
      self.z - other.z,
    )
  }
}

impl Mul for Vector3 {
  type Output = Self;
  
  fn mul(self, other: Self) -> Self::Output {
    Self::new(
      self.x * other.x,
      self.y * other.y,
      self.z * other.z,
    )
  }
}

impl Mul<f64> for Vector3 {
  type Output = Self;
  
  fn mul(self, other: f64) -> Self::Output {
    Self::new(
      self.x * other,
      self.y * other,
      self.z * other,
    )
  }
}

impl Mul<Vector3> for f64 {
  type Output = Vector3;
  
  fn mul(self, other: Vector3) -> Self::Output {
    other * self
  }
}

impl Div<f64> for Vector3 {
  type Output = Self;

  fn div(self, other: f64) -> Self::Output {
    if other == 0.0 { 
      self
    } else {
      (1.0/other) * self
    }
  }
}

pub fn dot(a: Vector3, b: Vector3) -> f64 {
  a.x*b.x + a.y*b.y + a.z*b.z
}

pub fn cross(a: Vector3, b: Vector3) -> Vector3 {
  Vector3::new(
    a.y * b.z - a.z * b.y,
    a.z * b.x - a.x * b.z,
    a.x * b.y - a.y * b.x,
  )
}

#[cfg(test)]
mod tests {
  use super::*;
  use rstest::*;

  mod vector3 {
    use super::*;

    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[rstest]
    fn new() {
      let (x, y, z) = (1.0, 2.0, 3.0);
      let vecs = vec![
        Vector3::new(x, y, z),
        Vector3::from((x, y, z)),
        Vector3::from([x, y, z]),
      ];
      for vec in vecs {
        assert_eq!(vec.x, x);
        assert_eq!(vec.y, y);
        assert_eq!(vec.z, z);
      }
    }

    #[rstest]
    fn random() {
      let mut rng = ChaCha8Rng::seed_from_u64(4);
      let vec = Vector3::random(&mut rng);
      let expected = Vector3::new(0.7262862070899089, 0.06981785470816837, 0.8784922015580432);
      assert_eq!(vec, expected);
      assert!(0.0 <= vec.x && vec.x <= 1.0);
      assert!(0.0 <= vec.y && vec.y <= 1.0);
      assert!(0.0 <= vec.z && vec.z <= 1.0);
    }

    #[rstest]
    #[case(0.0, 1.0)]
    #[case(-1.0, 1.0)]
    #[case(-100.0, 100.0)]
    #[case(0.0, 0.0)]
    #[case(1000.0, 1000.0)]
    #[case(-5.0, 5.0)]
    #[case(0.4, 100.5)]
    #[case(-0.2, 1.4)]
    fn random_range(#[case] min: f64, #[case] max: f64) {
      let mut rng = ChaCha8Rng::seed_from_u64(4);
      let vec = Vector3::random_range(&mut rng, min, max);
      assert!(min <= vec.x && vec.x <= max);
      assert!(min <= vec.y && vec.y <= max);
      assert!(min <= vec.z && vec.z <= max);
    }

    #[rstest]
    fn random_in_unit_sphere() {
      let mut rng = ChaCha8Rng::seed_from_u64(4);
      for _ in 0..100 {
        let vec = Vector3::random_in_unit_sphere(&mut rng);
        assert!(vec.length_squared() < 1.0); // ensure x^2+y^2+z^2 < r^2
      }
    }

    #[rstest]
    fn random_unit_vector() {
      let approx_equal = |a: f64, b: f64, dp: u8| -> bool {
        let p = 10f64.powi(-(dp as i32));
        (a-b).abs() < p
      };

      let mut rng = ChaCha8Rng::seed_from_u64(4);
      for _ in 0..100 {
        let vec = Vector3::random_unit_vector(&mut rng);
        // ensure vector is normalized - len=1, sits on unit circle, not just inside
        assert!(approx_equal(vec.length_squared(), 1.0, 8));
      }
    }

    #[rstest]
    fn random_on_hemisphere() {
      let mut rng = ChaCha8Rng::seed_from_u64(4);
      let cases = vec![
        (Vector3::new(0.0, 1.0, 0.0), Vector3::new(0.9591020070758018, 0.04373884536535061, -0.2796609615753949)),
        (Vector3::new(1.0, 0.0, 0.0), Vector3::new(0.36585754719776825, -0.5905345944631576, 0.7193171400020069)),
        (Vector3::new(0.3, 0.4, 0.8), Vector3::new(0.8572096589838902, -0.486776053098652, 0.16804961967948961)),
        (Vector3::new(0.0, 1.0, 1.0), Vector3::new(0.0430594550965176, 0.9989091454717781, -0.01806661062936974)),
        (Vector3::new(1.0, 1.0, 1.0), Vector3::new(0.40120317108123704, -0.7967771411915138, 0.45186524848569454)),
      ];
      for (normal, expected) in cases {
        let vec = Vector3::random_on_hemisphere(&mut rng, normal);
        assert_eq!(vec, expected);
      }
    }

    #[rstest]
    #[case(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0))]
    #[case(Vector3::new(1.0, 1.0, 1.0), Vector3::new(0.5773502691896258, 0.5773502691896258, 0.5773502691896258))]
    fn into_unit(#[case] vec: Vector3, #[case] expected: Vector3) {
      assert_eq!(vec.into_unit(), expected);
    }

    #[rstest]
    #[case(Vector3::new(0.0, 0.0, 0.0), 0.0)]
    #[case(Vector3::new(1.0, 0.0, 1.0), (1.0*1.0 + 1.0*1.0_f64).sqrt())]
    #[case(Vector3::new(2.0, 2.0, 2.0), (2.0*2.0 + 2.0*2.0 + 2.0*2.0_f64).sqrt())]
    fn length(#[case] vec: Vector3, #[case] expected: f64) {
      assert_eq!(vec.length(), expected);
    }

    #[rstest]
    fn length_squared() {
      let vec = Vector3::new(2.0, 2.0, 2.0);
      let expected = 2.0*2.0 + 2.0*2.0 + 2.0*2.0;
      assert_eq!(vec.length_squared(), expected);
    }

    #[rstest]
    fn neg() {
      let vec = Vector3::new(1.0, 1.0, -2.0);
      let expected = Vector3::new(-1.0, -1.0, 2.0);
      assert_eq!(-vec, expected);
    }

    #[rstest]
    #[case(0)]
    #[case(1)]
    #[case(2)]
    #[should_panic]
    #[case(3)]
    fn index_usize(#[case] index: usize) {
      let vec = Vector3::new(1.0, 1.0, 1.0);
      let expected = 1.0;
      assert_eq!(vec[index], expected);
    }

    #[rstest]
    #[case(0)]
    #[case(1)]
    #[case(2)]
    #[should_panic]
    #[case(3)]
    fn index_mut_usize(#[case] index: usize) {
      let mut vec = Vector3::new(1.0, 1.0, 1.0);
      let expected = 1.0 + 1.0;
      vec[index] += 1.0;
      assert_eq!(vec[index], expected);
    }

    #[rstest]
    #[case(Vector3::new(1.0, 1.0, 1.0), Vector3::new(1.0, 1.0, 1.0))]
    #[case(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0))]
    #[case(Vector3::new(1.0, -1.0, 0.0), Vector3::new(1.0, -1.0, 0.0))]
    #[case(Vector3::new(5.0, 0.0, 0.0), Vector3::new(5.0, 0.0, 0.0))]
    fn add_assign(#[case] addition: Vector3, #[case] expected: Vector3) {
      let mut vec = Vector3::new(0.0, 0.0, 0.0);
      vec += addition;
      assert_eq!(vec, expected);
    }

    #[rstest]
    #[case(0.0, Vector3::new(0.0, 0.0, 0.0))]
    #[case(1.0, Vector3::new(1.0, 1.0, 1.0))]
    #[case(2.0, Vector3::new(2.0, 2.0, 2.0))]
    #[case(-2.0, Vector3::new(-2.0, -2.0, -2.0))]
    fn mul_assign_f64(#[case] coefficient: f64, #[case] expected: Vector3) {
      let mut vec = Vector3::new(1.0, 1.0, 1.0);
      vec *= coefficient;
      assert_eq!(vec, expected);
    }

    #[rstest]
    #[case(1.0, Vector3::new(1.0, 1.0, 1.0))]
    #[case(2.0, Vector3::new(0.5, 0.5, 0.5))]
    #[case(0.5, Vector3::new(2.0, 2.0, 2.0))]
    #[case(-0.5, Vector3::new(-2.0, -2.0, -2.0))]
    #[should_panic]
    #[case(0.0, Vector3::new(0.0, 0.0, 0.0))]
    fn div_assign_f64(#[case] coefficient: f64, #[case] expected: Vector3) {
      let mut vec = Vector3::new(1.0, 1.0, 1.0);
      vec /= coefficient;
      assert_eq!(vec, expected);
    }
  
    #[rstest]
    #[case((0.0, 0.0, 0.0), "vec3(0, 0, 0)")]
    #[case((1.0, 2.0, 3.0), "vec3(1, 2, 3)")]
    #[case((0.1, 0.2, 0.3), "vec3(0.1, 0.2, 0.3)")]
    #[case((45.333, 2.81, 6.79), "vec3(45.333, 2.81, 6.79)")]
    fn display(#[case] tuple: (f64, f64, f64), #[case] expected: &str) {
      let vec = Vector3::from(tuple);
      assert_eq!(vec.to_string(), expected);
    }

    #[rstest]
    #[case(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0), Vector3::new(1.0, 1.0, 1.0))]
    #[case(Vector3::new(0.0, 1.0, 0.0), Vector3::new(1.0, 0.0, 1.0), Vector3::new(1.0, 1.0, 1.0))]
    #[case(Vector3::new(1.0, 1.0, 1.0), Vector3::new(-1.0, -1.0, -1.0), Vector3::new(0.0, 0.0, 0.0))]
    #[case(Vector3::new(1.0, 1.0, 1.0), Vector3::new(1.0, 1.0, 1.0), Vector3::new(2.0, 2.0, 2.0))]
    fn add(#[case] a: Vector3, #[case] b: Vector3, #[case] expected: Vector3) {
      assert_eq!(a + b, expected);
    }

    #[rstest]
    #[case(Vector3::new(1.0, 1.0, 1.0), Vector3::new(1.0, 1.0, 1.0), Vector3::new(0.0, 0.0, 0.0))]
    #[case(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0), Vector3::new(-1.0, -1.0, -1.0))]
    #[case(Vector3::new(0.0, 1.0, 0.0), Vector3::new(1.0, 0.0, 1.0), Vector3::new(-1.0, 1.0, -1.0))]
    #[case(Vector3::new(1.0, 1.0, 1.0), Vector3::new(-1.0, -1.0, -1.0), Vector3::new(2.0, 2.0, 2.0))]
    fn sub(#[case] a: Vector3, #[case] b: Vector3, #[case] expected: Vector3) {
      assert_eq!(a - b, expected);
    }

    #[rstest]
    #[case(Vector3::new(1.0, 1.0, 1.0), 0.0, Vector3::new(0.0, 0.0, 0.0))]
    #[case(Vector3::new(1.0, 2.0, 0.0), 1.0, Vector3::new(1.0, 2.0, 0.0))]
    #[case(Vector3::new(0.0, 1.0, 0.0), 2.0, Vector3::new(0.0, 2.0, 0.0))]
    #[case(Vector3::new(1.0, 1.0, 1.0), -1.0, Vector3::new(-1.0, -1.0, -1.0))]
    fn mul_f64_f64_mul(#[case] a: Vector3, #[case] coefficient: f64, #[case] expected: Vector3) {
      assert_eq!(a * coefficient, expected);
      assert_eq!(coefficient * a, expected);
    }

    #[rstest]
    #[case(Vector3::new(1.0, 1.0, 1.0), Vector3::new(1.0, 1.0, 1.0), 3.0)]
    #[case(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0), 0.0)]
    #[case(Vector3::new(-1.0, 1.0, 0.0), Vector3::new(1.0, 1.0, 42.0), 0.0)]
    #[case(Vector3::new(3.0, 4.0, 0.0), Vector3::new(1.0, 1.0, 1.0), 3.0+4.0)]
    fn dot(#[case] a: Vector3, #[case] b: Vector3, #[case] expected: f64) {
      assert_eq!(super::dot(a, b), expected);
    }

    #[rstest]
    #[case(Vector3::new(1.0, 1.0, 1.0), Vector3::new(1.0, 1.0, 1.0), Vector3::new(0.0, 0.0, 0.0))]
    #[case(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 1.0), Vector3::new(0.0, 0.0, 0.0))]
    #[case(Vector3::new(-1.0, 1.0, 0.0), Vector3::new(1.0, 1.0, 42.0), Vector3::new(42.0, 42.0, -2.0))]
    #[case(Vector3::new(3.0, 4.0, 0.0), Vector3::new(1.0, 1.0, 1.0), Vector3::new(4.0, -3.0, -1.0))]
    fn cross(#[case] a: Vector3, #[case] b: Vector3, #[case] expected: Vector3) {
      assert_eq!(super::cross(a, b), expected);
    }
  }
}