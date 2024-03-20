
use criterion::{criterion_group, criterion_main, Criterion};

use lib_raytracer::prelude::*; // external crate; pub fns only

use simulation::png::Png;

mod bench_png {
  use super::*;

  pub fn render(c: &mut Criterion) {
    let mut group = c.benchmark_group("png_render");
    group.sample_size(10);
    group.bench_function("Png::render", |b| {
      let (mut camera, mut hittable) = simulation::generate_world();
      let mut png = Png::new();
      b.iter(|| png.render(&mut camera, &mut hittable)) 
    });
    group.finish();
  }
}

criterion_group!(benches_png, 
  bench_png::render, 
);
criterion_main!(benches_png);
