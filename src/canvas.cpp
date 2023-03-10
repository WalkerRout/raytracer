
#include <iostream>
#include <sstream>
#include <fstream>
#include <cmath>
#include <ctime>
#include <cassert>
#include <cstdlib>

#include "canvas.h"

namespace canvas {

auto Color::black(void) -> Color {
  return Color({0, 0, 0});
}

auto Color::white(void) -> Color {
  return Color({1, 1, 1});
}

auto Color::red(void) -> Color {
  return Color({1, 0, 0});
}

auto Color::green(void) -> Color {
  return Color({0, 1, 0});
}

auto Color::blue(void) -> Color {
  return Color({0, 0, 1}); 
}

auto Color::random(void) -> Color {
  // color bound is [p, q]
  double p = 0;
  double q = 1;

  double r = std::rand();
  double g = std::rand();
  double b = std::rand();

  // linear interpolation to map range of [0, RAND_MAX] to [p, q]
  r = p + ((q - p) / RAND_MAX) * r;
  g = p + ((q - p) / RAND_MAX) * g;
  b = p + ((q - p) / RAND_MAX) * b;

  return Color({(double)r, (double)g, (double)b});
}

auto Color::operator+(Color rhs) const -> Color {
  return Color({rgb[0] + rhs.rgb[0], rgb[1] + rhs.rgb[1], rgb[2] + rhs.rgb[2]});
}

auto Color::operator-(Color rhs) const -> Color {
  return Color({rgb[0] - rhs.rgb[0], rgb[1] - rhs.rgb[1], rgb[2] - rhs.rgb[2]});
}

auto Color::operator*(Color rhs) const -> Color {
  return Color({rgb[0] * rhs.rgb[0], rgb[1] * rhs.rgb[1], rgb[2] * rhs.rgb[2]});
}

auto Color::operator*(double scalar) const -> Color {
  return Color({rgb[0] * scalar, rgb[1] * scalar, rgb[2] * scalar});
}

auto Color::operator==(Color rhs) const -> bool {
  auto result = true;
  result &= rhs.rgb[0] == rgb[0];
  result &= rhs.rgb[1] == rgb[1];
  result &= rhs.rgb[2] == rgb[2];
  return result;
}

auto Color::string() const -> std::string {
  auto scale = [](double x, double mul) -> double {
    return std::clamp(round(x * mul), 0., mul);
  };
  auto oss = std::ostringstream();
  oss << scale(rgb[0], 255) << " " << scale(rgb[1], 255) << " " << scale(rgb[2], 255);
  return oss.str();
}

auto Canvas::set_pixel(size_t x, size_t y, Color c) -> void {
  if(0 <= x && x < width && 0 <= y && y < height) {
    buffer[y * width + x] = c;
  }
  // else ignore
}

auto Canvas::get_pixel(size_t x, size_t y) const -> Color {
  assert(0 <= x && x < width && "Width is out of bounds!");
  assert(0 <= y && y < height && "Height is out of bounds!");
  return buffer[y * width + x];
}

auto Canvas::set_background(Color c) -> void {
  for(auto& pixel : buffer) {
    pixel = c;
  }
}

auto Canvas::to_ppm(void) const -> std::string {
  auto space_ppm_newline = [](std::string s) {
    auto str_len = s.length();
    auto str_stream = std::stringstream();
    auto insert_newline = false;

    if(str_len > 70) { // ... replace next space with a newline, and then repeat every time i % 70 == 0
      str_stream << s[0];

      for(int i = 1; i < str_len; ++i) {
        auto curr_char = s[i];
        if(i % 67 == 0) { // 70 is max length, each section is max 3 characters, 70 - 3 = 67 for exact constraint
          insert_newline = true;
        }

        if(insert_newline && curr_char == ' ') {
          str_stream << '\n';
          insert_newline = false;
        } else {
          str_stream << s[i];
        }
      }

    } else {
      return s;
    }

    return str_stream.str();
  };
  // find how to reserve string space, count_elements * 3 + count_elements (spaces)
  auto header = std::string("P3\n") + 
    std::to_string((unsigned long)width) + 
    " " + 
    std::to_string((unsigned long)height) + 
    "\n255";

  // this is gonna be slow...
  std::stringstream temp;
  std::stringstream result;

  result << header;
  result << '\n';

  for(auto i = 1; auto& pixel : buffer) {
    temp << pixel.string();

    if(i % width == 0) {
      result << space_ppm_newline(temp.str());
      result << '\n';
      temp = std::stringstream();
    } else {
      temp << ' ';
    }

    ++i;
  }

  result.seekp(-1, std::ios_base::end);
  if(result.peek() == '\n') {
    return result.str();
  }

  result << '\n';
  return result.str();
}

auto Canvas::to_disk(std::string file_name) const -> void {
  auto file = std::ofstream(file_name);
  file << to_ppm();
  file.close();
}

}