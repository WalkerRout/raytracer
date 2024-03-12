
#[allow(unused_imports)]
use crate::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Interval {
  pub min: f64,
  pub max: f64,
}

impl Interval {
  pub const fn new(min: f64, max: f64) -> Self {
    Self { min, max, }
  }

  pub fn contains(&self, x: f64) -> bool {
    self.min <= x && x <= self.max
  }

  pub fn surrounds(&self, x: f64) -> bool {
    self.min < x && x < self.max
  }

  pub fn clamp(&self, x: f64) -> f64 {
    x.clamp(self.min, self.max)
  }
}

impl Default for Interval {
  fn default() -> Self {
    Self {
      min: f64::NEG_INFINITY,
      max: f64::INFINITY,
    }
  }
}

pub const EMPTY: Interval = Interval::new(f64::INFINITY, f64::NEG_INFINITY);
pub const UNIVERSE: Interval = Interval::new(f64::NEG_INFINITY, f64::INFINITY);

#[cfg(test)]
mod tests {
  use super::*;
  use rstest::*;

  mod interval {
    use super::*;

    #[rstest]
    #[case(Interval::new(0.0, 0.0), 0.0, 0.0)]
    #[case(Interval::new(-1.0, 0.0), -1.0, 0.0)]
    #[case(Interval::new(5.0, 0.0), 5.0, 0.0)]
    #[case(Interval::new(f64::NEG_INFINITY, f64::INFINITY), f64::NEG_INFINITY, f64::INFINITY)]
    #[should_panic] // behaviour not yet defined for f64::NAN, since Interval::new called in hot loops
    #[case(Interval::new(f64::NAN, f64::NAN), f64::NAN, f64::NAN)]
    fn new(#[case] interval: Interval, #[case] expected_min: f64, #[case] expected_max: f64) {
      assert_eq!(interval.min, expected_min);
      assert_eq!(interval.max, expected_max);
    }

    #[rstest]
    fn default() {
      let interval = Interval::default();
      assert_eq!(interval.min, f64::NEG_INFINITY);
      assert_eq!(interval.max, f64::INFINITY);
      assert_eq!(interval, UNIVERSE);
    }
  }
}