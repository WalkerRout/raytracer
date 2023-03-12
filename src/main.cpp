
#include <iostream>
#include <ctime>
#include <cstdlib>

#include "raymath.h"
#include "canvas.h"
#include "launch.h"

auto main() -> int {
  std::srand(std::time(0));

  auto canvas = canvas::Canvas(900, 550);
  canvas.set_background(canvas::Color::white());

  auto start = raymath::Point({0, 1, 0});
  auto velocity = raymath::Vector({1, 1.8, 0}).normalize() * 11.25;
  std::cout << raymath::Vector({1.6, 1.8, 0}).normalize() << '\n';
  auto projectile = launch::Projectile(start, velocity);

  auto gravity = raymath::Vector({0, -0.1, 0});
  auto wind_speed = raymath::Vector({-0.01, 0, 0});
  auto environment = launch::Environment(gravity, wind_speed);

  while(projectile.position.xyzw.get_data_ref()[1] > 0) {
    projectile = tick(environment, projectile);
    canvas.set_pixel(
      projectile.position.xyzw.get_data_ref()[0], 
      550 - projectile.position.xyzw.get_data_ref()[1], 
      canvas::Color::black()
    );
  }

  canvas.to_disk("test.ppm");

  return 0;
}