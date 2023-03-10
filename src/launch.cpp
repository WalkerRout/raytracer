
#include "launch.h"

namespace launch {

auto tick(Environment env, Projectile proj) -> Projectile {
  auto position = proj.position + proj.velocity;
  auto velocity = proj.velocity + env.gravity + env.wind;
  return Projectile(position, velocity);
}

}