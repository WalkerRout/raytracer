
#include <array>
#include <cmath>

#include "gtest/gtest.h"

#include "canvas.h"

TEST(test_canvas, test_color) {
  auto color = canvas::Color({-0.5, 0.7, 1.5});

  auto test = std::array<double, 3>{-0.5, 0.7, 1.5};
  EXPECT_EQ(color.rgb, test);
}

TEST(test_canvas, test_color_plus_color) {
  auto color_a = canvas::Color({-0.5, 0.7, 1.5});
  auto color_b = canvas::Color({0.5, -0.7, -1.5});

  auto test = canvas::Color({0, 0, 0});
  EXPECT_EQ(color_a + color_b, test);
}

TEST(test_canvas, test_color_sub_color) {
  auto color_a = canvas::Color({0.5, 0.7, 1.5});
  auto color_b = canvas::Color({0.5, 0.7, 1.5});

  auto test = canvas::Color({0, 0, 0});
  EXPECT_EQ(color_a - color_b, test);
}

TEST(test_canvas, test_color_mul_color) {
  auto color_a = canvas::Color({1, 0.7, 0.25});
  auto color_b = canvas::Color({0.5, 1, 0.1});

  // this test only passes because comparing with 0.25 (representable as a double)
  auto test = canvas::Color({0.5, 0.7, 0.25 * 0.1});
  EXPECT_EQ(color_a * color_b, test);
}

TEST(test_canvas, test_color_mul_scalar) {
  auto color = canvas::Color({-0.5, 0.7, 1.5});
  auto scalar = 2;

  auto test = canvas::Color({-0.5 * scalar, 0.7 * scalar, 1.5 * scalar});
  EXPECT_EQ(color * scalar, test);
}

TEST(test_canvas, test_canvas) {
  auto canvas = canvas::Canvas(20, 20);

  EXPECT_EQ(canvas.width, 20);
  EXPECT_EQ(canvas.height, 20);
  EXPECT_EQ(canvas.buffer.size(), 20 * 20);
  EXPECT_EQ(canvas.buffer[10 * canvas.width + 10], canvas::Color::black());
}

TEST(test_canvas, test_canvas_set_get_pixel) {
  auto canvas = canvas::Canvas(20, 20);

  canvas.set_pixel(10, 10, canvas::Color::red());
  EXPECT_EQ(canvas.get_pixel(10, 10), canvas::Color::red());
}

TEST(test_canvas, test_canvas_to_ppm) {
  auto canvas = canvas::Canvas(15, 2);
  for(int i = 0; i < 15; ++i) {
    for(int j = 0; j < 2; ++j) {
      canvas.set_pixel(i, j, canvas::Color({0.77, 0.25, 0.3}));
    }
  }

  auto test = R"(P3
15 2
255
196 64 77 196 64 77 196 64 77 196 64 77 196 64 77 196 64 77 196 64 77
196 64 77 196 64 77 196 64 77 196 64 77 196 64 77 196 64 77 196 64
77 196 64 77
196 64 77 196 64 77 196 64 77 196 64 77 196 64 77 196 64 77 196 64 77
196 64 77 196 64 77 196 64 77 196 64 77 196 64 77 196 64 77 196 64
77 196 64 77
)";

  EXPECT_EQ(canvas.to_ppm(), test);
}