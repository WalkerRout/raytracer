#ifndef CANVAS_H
#define CANVAS_H

#include <array>
#include <vector>

namespace canvas {

struct Color {
  Color() = default;
  Color(std::array<double, 3> c):
    rgb(c) {}

  // Rule of Zero

  static auto black(void) -> Color;
  static auto white(void) -> Color;
  static auto red(void) -> Color;
  static auto green(void) -> Color;
  static auto blue(void) -> Color;
  static auto random(void) -> Color;

  auto operator+(Color rhs) const -> Color;
  auto operator-(Color rhs) const -> Color;
  auto operator*(Color rhs) const -> Color;
  auto operator*(double scalar) const -> Color;
  auto operator==(Color rhs) const -> bool;

  auto string() const -> std::string;

public:
  std::array<double, 3> rgb{0, 0, 0};
};

struct Canvas {
  Canvas() = default;
  Canvas(size_t w, size_t h):
    width(w),
    height(h) {
    buffer = std::move(std::vector<Color>(w * h));
  }

  // Rule of Zero

  // j=x, i=y
  auto set_pixel(size_t x, size_t y, Color c) -> void;
  auto get_pixel(size_t x, size_t y) const -> Color;
  auto set_background(Color c) -> void;
  auto to_ppm(void) const -> std::string;
  auto to_disk(std::string file_name) const -> void;

public:
  size_t width;
  size_t height;
//private: // comment for tests
  std::vector<Color> buffer;
};

}

#endif // CANVAS_H