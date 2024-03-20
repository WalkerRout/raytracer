
use criterion::{criterion_group, criterion_main, Criterion};

use lib_raytracer::prelude::*; // external crate; pub fns only

use simulation::ppm::Ppm;

mod bench_ppm {
  use super::*;

  pub fn render(c: &mut Criterion) {
    let mut group = c.benchmark_group("ppm_render");
    group.sample_size(10);
    group.bench_function("Ppm::render", |b| {
      let (mut camera, mut hittable) = simulation::generate_world();
      let mut ppm = Ppm::new();
      b.iter(|| ppm.render(&mut camera, &mut hittable)) 
    });
    group.finish();
  }
}

criterion_group!(benches_ppm, 
  bench_ppm::render, 
);
criterion_main!(benches_ppm);
