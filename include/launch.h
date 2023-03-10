#ifndef LAUNCH_H
#define LAUNCH_H

#include "raymath.h"

namespace launch {

struct Projectile {
  Projectile() = default;
  Projectile(raymath::Point p, raymath::Vector v):
    position(p),
    velocity(v) {}

public:
  raymath::Point position;
  raymath::Vector velocity;
};

struct Environment {
  Environment() = default;
  Environment(raymath::Vector w, raymath::Vector g):
    wind(w),
    gravity(g) {}

public:
  raymath::Vector wind;
  raymath::Vector gravity;
};

auto tick(Environment env, Projectile proj) -> Projectile;

}

#endif // LAUNCH_H