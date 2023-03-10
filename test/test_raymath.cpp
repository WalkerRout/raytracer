
#include <algorithm>
#include <cmath>

#include "gtest/gtest.h"

#include "raymath.h"
#include "raymath.cpp"

template <typename T>
static constexpr auto double_cmp(const T a, const T b, const T epsilon) -> bool {
  return std::abs(a - b) <= (std::max(a, b) * epsilon);
}

TEST(test_raymath, test_vector) {
  auto point = raymath::Vector({1, 2, 3});

  auto test = std::array<double, 4>{1, 2, 3, 0};
  EXPECT_EQ(point.xyzw, test);
}

TEST(test_raymath, test_vector_plus_vector) {
  auto vector_a = raymath::Vector({1, 2, 3});
  auto vector_b = raymath::Vector({1, 2, 3});

  auto test = raymath::Vector({2, 4, 6});
  EXPECT_EQ(vector_a + vector_b, test);
}

TEST(test_raymath, test_vector_plus_point) {
  auto point = raymath::Point({1, 2, 3});
  auto vector = raymath::Vector({1, 2, 3});

  auto test = raymath::Point({2, 4, 6});
  EXPECT_EQ(vector + point, test);
}

TEST(test_raymath, test_negate_vector) {
  auto vector = raymath::Vector({1, 2, 3});

  auto test = raymath::Vector({-1, -2, -3});
  EXPECT_EQ(-vector, test);
}

TEST(test_raymath, test_vector_sub_vector) {
  auto vector_a = raymath::Vector({1, 2, 3});
  auto vector_b = raymath::Vector({1, 2, 3});

  auto test = raymath::Vector({0, 0, 0});
  EXPECT_EQ(vector_a - vector_b, test);
}

TEST(test_raymath, test_vector_sub_zero) {
  auto vector_a = raymath::Vector::zero();
  auto vector_b = raymath::Vector({1, 2, 3});

  auto test = raymath::Vector({-1, -2, -3});
  EXPECT_EQ(vector_a - vector_b, test);
}

TEST(test_raymath, test_vector_mul_scalar) {
  auto vector = raymath::Vector({1, 2, 3});
  auto scalar = 2;

  auto test = raymath::Vector({scalar * 1, scalar * 2, scalar * 3});
  EXPECT_EQ(vector * scalar, test);
}

TEST(test_raymath, test_vector_div_scalar) {
  auto vector = raymath::Vector({4, 4, 4});
  auto scalar = 2.0;

  auto test = raymath::Vector({4 / scalar, 4 / scalar, 4 / scalar});
  EXPECT_EQ(vector / scalar, test);
}

TEST(test_raymath, test_vector_dot_vector) {
  auto vector_a = raymath::Vector({1, 2, 3});
  auto vector_b = raymath::Vector({2, 3, 4});

  auto test = 20;
  EXPECT_EQ(vector_a.dot(vector_b), test);
}

TEST(test_raymath, test_vector_cross_vector) {
  auto vector_a = raymath::Vector({1, 2, 3});
  auto vector_b = raymath::Vector({2, 3, 4});

  auto test_ab = raymath::Vector({-1, 2, -1});
  auto test_ba = raymath::Vector({1, -2, 1});
  EXPECT_EQ(vector_a.cross(vector_b), test_ab);
  EXPECT_EQ(vector_b.cross(vector_a), test_ba);
}

TEST(test_raymath, test_vector_magnitude) {
  auto vector_a = raymath::Vector({1, 2, 3});
  auto vector_b = raymath::Vector({-1, -2, -3});

  auto test = 14;
  // scuffed cast to double since sqrt returns double in order to do double comparison
  EXPECT_EQ(vector_a.magnitude(), (double)std::sqrt(test));
  EXPECT_EQ(vector_b.magnitude(), (double)std::sqrt(test));
}

TEST(test_raymath, test_vector_normalize) {
  auto vector_a = raymath::Vector({1, 0, 0});
  auto vector_b = raymath::Vector({4, 0, 0});
  auto vector_c = raymath::Vector({1, 2, 3});

  auto magn_c = vector_c.magnitude();
  auto test_ab = raymath::Vector({1, 0, 0});
  auto test_c = raymath::Vector({1 / magn_c, 2 / magn_c, 3 / magn_c});
  EXPECT_EQ(vector_a.normalize(), test_ab);
  EXPECT_EQ(vector_b.normalize(), test_ab);
  EXPECT_EQ(vector_c.normalize(), test_c);
  EXPECT_TRUE(double_cmp<double>(vector_c.normalize().magnitude(), 1, std::numeric_limits<double>::epsilon()));
}

TEST(test_raymath, test_point) {
  auto point = raymath::Point({1, 2, 3});

  auto test = std::array<double, 4>{1, 2, 3, 1};
  EXPECT_EQ(point.xyzw, test);
}

TEST(test_raymath, test_point_plus_vector) {
  auto point = raymath::Point({1, 2, 3});
  auto vector = raymath::Vector({1, 2, 3});

  auto test = raymath::Point({2, 4, 6});
  EXPECT_EQ(point + vector, test);
}

TEST(test_raymath, test_negate_point) {
  auto point = raymath::Point({1, 2, 3});

  auto test = raymath::Point({-1, -2, -3});
  EXPECT_EQ(-point, test);
}

TEST(test_raymath, test_point_sub_point) {
  auto point_a = raymath::Point({1, 2, 3});
  auto point_b = raymath::Point({1, 2, 3});

  auto test = raymath::Vector({0, 0, 0});
  EXPECT_EQ(point_a - point_b, test);
}

TEST(test_raymath, test_point_sub_vector) {
  auto point = raymath::Point({1, 2, 3});
  auto vector = raymath::Vector({1, 2, 3});

  auto test = raymath::Point({0, 0, 0});
  EXPECT_EQ(point - vector, test);
}

TEST(test_raymath, test_point_mul_scalar) {
  auto point = raymath::Point({1, 2, 3});
  auto scalar = 2.0;

  auto test = raymath::Point({scalar * 1, scalar * 2, scalar * 3});
  EXPECT_EQ(point * scalar, test);
}

TEST(test_raymath, test_point_div_scalar) {
  auto point = raymath::Point({4, 4, 4});
  auto scalar = 2.0;

  auto test = raymath::Point({4 / scalar, 4 / scalar, 4 / scalar});
  EXPECT_EQ(point / scalar, test);
}

TEST(test_raymath, test_matrix_2x2) {
  auto matrix = raymath::Matrix<2, 2>(
    1., 2.,
    2., 1.
  );

  EXPECT_EQ(matrix.get(0, 1), 2.);
  EXPECT_EQ(matrix.get_data()[0 * 2 + 1], 2.);
}

TEST(test_raymath, test_matrix_3x3) {
  auto matrix = raymath::Matrix<3, 3>(
    1., 2., 3.,
    4., 5., 4.,
    3., 2., 1.
  );

  EXPECT_EQ(matrix.get(1, 1), 5.);
  EXPECT_EQ(matrix.get_data()[1 * 3 + 1], 5.);
}

TEST(test_raymath, test_matrix_4x4) {
  auto matrix = raymath::Matrix<4, 4>(
    1., 2., 3., 4.,
    5., 6., 7., 8.,
    8., 7., 6., 5.,
    4., 3., 2., 1.
  );

  EXPECT_EQ(matrix.get(2, 3), 5.);
  EXPECT_EQ(matrix.get_data()[2 * 4 + 3], 5.);
}