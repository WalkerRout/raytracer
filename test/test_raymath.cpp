
#include <algorithm>
#include <cassert>
#include <cmath>

#include "gtest/gtest.h"

#define STACK_LOC_CUTOFF (128*128) // for the purposes of consistency within tests
#include "raymath.h"
#undef  STACK_LOC_CUTOFF // dont interfere with anyplace else it is defined

TEST(test_raymath, test_vector) {
  auto point = raymath::Vector({1., 2., 3.});

  auto test = raymath::Matrix<4, 1>{1., 2., 3., 0.};
  EXPECT_EQ(point.xyzw, test);
}

TEST(test_raymath, test_vector_plus_vector) {
  auto vector_a = raymath::Vector({1., 2., 3.});
  auto vector_b = raymath::Vector({1., 2., 3.});

  auto test = raymath::Vector({2., 4., 6.});
  EXPECT_EQ(vector_a + vector_b, test);
}

TEST(test_raymath, test_vector_plus_point) {
  auto point = raymath::Point({1., 2., 3.});
  auto vector = raymath::Vector({1., 2., 3.});

  auto test = raymath::Point({2., 4., 6.});
  EXPECT_EQ(vector + point, test);
}

TEST(test_raymath, test_negate_vector) {
  auto vector = raymath::Vector({1., 2., 3.});

  auto test = raymath::Vector({-1., -2., -3.});
  EXPECT_EQ(-vector, test);
}

TEST(test_raymath, test_vector_sub_vector) {
  auto vector_a = raymath::Vector({1., 2., 3.});
  auto vector_b = raymath::Vector({1., 2., 3.});

  auto test = raymath::Vector({0., 0., 0.});
  EXPECT_EQ(vector_a - vector_b, test);
}

TEST(test_raymath, test_vector_sub_zero) {
  auto vector_a = raymath::Vector::zero();
  auto vector_b = raymath::Vector({1., 2., 3.});

  auto test = raymath::Vector({-1., -2., -3.});
  EXPECT_EQ(vector_a - vector_b, test);
}

TEST(test_raymath, test_vector_mul_scalar) {
  auto vector = raymath::Vector({1., 2., 3.});
  auto scalar = 2;

  auto test = raymath::Vector({scalar * 1., scalar * 2., scalar * 3.});
  EXPECT_EQ(vector * scalar, test);
}

TEST(test_raymath, test_vector_div_scalar) {
  auto vector = raymath::Vector({4., 4., 4.});
  auto scalar = 2.0;

  auto test = raymath::Vector({4. / scalar, 4. / scalar, 4. / scalar});
  EXPECT_EQ(vector / scalar, test);
}

TEST(test_raymath, test_vector_dot_vector) {
  auto vector_a = raymath::Vector({1., 2., 3.});
  auto vector_b = raymath::Vector({2., 3., 4.});

  auto test = 20.;
  EXPECT_EQ(vector_a.dot(vector_b), test);
}

TEST(test_raymath, test_vector_cross_vector) {
  auto vector_a = raymath::Vector({1., 2., 3.});
  auto vector_b = raymath::Vector({2., 3., 4.});

  auto test_ab = raymath::Vector({-1., 2., -1.});
  auto test_ba = raymath::Vector({1., -2., 1.});
  EXPECT_EQ(vector_a.cross(vector_b), test_ab);
  EXPECT_EQ(vector_b.cross(vector_a), test_ba);
}

TEST(test_raymath, test_vector_magnitude) {
  auto vector_a = raymath::Vector({1., 2., 3.});
  auto vector_b = raymath::Vector({-1., -2., -3.});

  auto test = 14.;
  // scuffed cast to double since sqrt returns double in order to do double comparison
  EXPECT_EQ(vector_a.magnitude(), (double)std::sqrt(test));
  EXPECT_EQ(vector_b.magnitude(), (double)std::sqrt(test));
}

TEST(test_raymath, test_vector_normalize) {
  auto vector_a = raymath::Vector({1., 0., 0.});
  auto vector_b = raymath::Vector({4., 0., 0.});
  auto vector_c = raymath::Vector({1., 2., 3.});

  auto magn_c = vector_c.magnitude();
  auto test_ab = raymath::Vector({1., 0., 0.});
  auto test_c = raymath::Vector({1. / magn_c, 2. / magn_c, 3. / magn_c});
  EXPECT_EQ(vector_a.normalize(), test_ab);
  EXPECT_EQ(vector_b.normalize(), test_ab);
  EXPECT_EQ(vector_c.normalize(), test_c);
  EXPECT_TRUE(raymath::eqf<double>(vector_c.normalize().magnitude(), 1., std::numeric_limits<double>::epsilon()));
}

TEST(test_raymath, test_point) {
  auto point = raymath::Point({1., 2., 3.});

  auto test = raymath::Matrix<4, 1>{1., 2., 3., 1.};
  EXPECT_EQ(point.xyzw, test);
}

TEST(test_raymath, test_point_plus_vector) {
  auto point = raymath::Point({1., 2., 3.});
  auto vector = raymath::Vector({1., 2., 3.});

  auto test = raymath::Point({2., 4., 6.});
  EXPECT_EQ(point + vector, test);
}

TEST(test_raymath, test_negate_point) {
  auto point = raymath::Point({1., 2., 3.});

  auto test = raymath::Point({-1., -2., -3.});
  EXPECT_EQ(-point, test);
}

TEST(test_raymath, test_point_sub_point) {
  auto point_a = raymath::Point({1., 2., 3.});
  auto point_b = raymath::Point({1., 2., 3.});

  auto test = raymath::Vector({0., 0., 0.});
  EXPECT_EQ(point_a - point_b, test);
}

TEST(test_raymath, test_point_sub_vector) {
  auto point = raymath::Point({1., 2., 3.});
  auto vector = raymath::Vector({1., 2., 3.});

  auto test = raymath::Point({0., 0., 0.});
  EXPECT_EQ(point - vector, test);
}

TEST(test_raymath, test_point_mul_scalar) {
  auto point = raymath::Point({1., 2., 3.});
  auto scalar = 2.;

  auto test = raymath::Point({scalar * 1., scalar * 2., scalar * 3.});
  EXPECT_EQ(point * scalar, test);
}

TEST(test_raymath, test_point_div_scalar) {
  auto point = raymath::Point({4., 4., 4.});
  auto scalar = 2.;

  auto test = raymath::Point({4. / scalar, 4. / scalar, 4. / scalar});
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

TEST(test_raymath, test_matrix_130x130) {
  auto matrix = raymath::Matrix<130, 130>(); // allocated on the heap
  matrix.set(129, 129, 42.);

  EXPECT_EQ(matrix.on_heap(), true);
  EXPECT_EQ(matrix.get(129, 129), 42.);
}

TEST(test_raymath, test_matrix4x4_add_matrix4x4) {
  auto matrix_a = raymath::Matrix<4, 4>(1., 2., 3., 4., 5.);
  auto matrix_b = raymath::Matrix<4, 4>(1., 2., 3., 4., 5.);

  auto test = raymath::Matrix<4, 4>(2., 4., 6., 8., 10.);
  EXPECT_EQ(matrix_a + matrix_b, test);
}

TEST(test_raymath, test_matrix4x4_sub_matrix4x4) {
  auto matrix_a = raymath::Matrix<4, 4>(1., 2., 3., 4., 5.);
  auto matrix_b = raymath::Matrix<4, 4>(1., 2., 3., 4., 5.);

  auto test = raymath::Matrix<4, 4>();
  EXPECT_EQ(matrix_a - matrix_b, test);
}

TEST(test_raymath, test_matrix4x4_div_matrix4x4) {
  auto matrix_a = raymath::Matrix<4, 4>();
  auto matrix_b = raymath::Matrix<4, 4>(
    1., 1., 1., 1.,
    1., 1., 1., 1.,
    1., 1., 1., 1.,
    1., 1., 1., 1.
  );

  auto test = raymath::Matrix<4, 4>();
  EXPECT_EQ(matrix_a / matrix_b, test);
}

TEST(test_raymath, test_matrix4x4_hadamard_matrix4x4) {
  auto matrix_a = raymath::Matrix<4, 4>(1., 2., 3., 4., 5.);
  auto matrix_b = raymath::Matrix<4, 4>(1., 2., 3., 4., 5.);

  auto test = raymath::Matrix<4, 4>(1., 4., 9., 16., 25.);
  EXPECT_EQ(matrix_a.hadamard(matrix_b), test);
}

TEST(test_raymath, test_matrix4x4_dot_matrix4x4) {
  auto matrix_a = raymath::Matrix<4, 4>(
    1., 2., 3., 4.,
    5., 6., 7., 8.,
    9., 8., 7., 6.,
    5., 4., 3., 2.
  );
  auto matrix_b = raymath::Matrix<4, 4>(
    -2., 1., 2.,  3.,
     3., 2., 1., -1.,
     4., 3., 6.,  5.,
     1., 2., 7.,  8.
  );

  auto test = raymath::Matrix<4, 4>(
    20., 22., 50.,  48.,
    44., 54., 114., 108.,
    40., 58., 110., 102.,
    16., 26., 46.,  42.
  );
  EXPECT_EQ(matrix_a * matrix_b, test);
}

// heap * stack
TEST(test_raymath, test_matrix130x130_dot_matrix130x5) {
  auto matrix_a = raymath::Matrix<130, 130>();
  auto matrix_b = raymath::Matrix<130, 5>();

  // make sure that heap allocated matrices and stack allocated matrices have same working API
  auto test = raymath::Matrix<130, 5>();
  EXPECT_EQ(matrix_a * matrix_b, test);
}

TEST(test_raymath, test_stack_matrix_transpose) {
  auto matrix_a = raymath::Matrix<4, 4>();
  auto matrix_b = raymath::Matrix<12, 8>();

  auto test_a = raymath::Matrix<4, 4>();
  auto test_b = raymath::Matrix<8, 12>();
  EXPECT_EQ(matrix_a.transpose(), test_a);
  EXPECT_EQ(matrix_b.transpose(), test_b);
}

TEST(test_raymath, test_heap_matrix_transpose) {
  auto matrix_a = raymath::Matrix<130, 130>();
  auto matrix_b = raymath::Matrix<140, 130>();

  auto test_a = raymath::Matrix<130, 130>();
  auto test_b = raymath::Matrix<130, 140>();
  EXPECT_EQ(matrix_a.transpose(), test_a);
  EXPECT_EQ(matrix_b.transpose(), test_b);
}

TEST(test_raymath, test_identity_matrix) {
  auto matrix = raymath::Matrix<4, 4>::identity();

  auto test = raymath::Matrix<4, 4>(
    1., 0., 0., 0.,
    0., 1., 0., 0.,
    0., 0., 1., 0.,
    0., 0., 0., 1.
  );
  EXPECT_EQ(matrix, test);
}

TEST(test_raymath, test_matrix_dot_identity_matrix) {
  auto matrix_a = raymath::Matrix<4, 4>(1., 2., 3., 4.);
  auto matrix_b = raymath::Matrix<4, 4>::identity();

  auto test = raymath::Matrix<4, 4>(1., 2., 3., 4.); // same as matrix_a
  EXPECT_EQ(matrix_a * matrix_b, test);
}

TEST(test_raymath, test_matrix_dot_vector) {
  auto matrix = raymath::Matrix<4, 4>(1.);
  auto vector = raymath::Vector({1., 2., 3.});

  auto test = raymath::Matrix<4, 1>(1.);
  EXPECT_EQ(matrix * vector.xyzw, test);
}

TEST(test_raymath, test_matrix_dot_point) {
  auto matrix = raymath::Matrix<4, 4>(1.);
  auto point = raymath::Point({1., 2., 3.});

  auto test = raymath::Matrix<4, 1>(1.);
  EXPECT_EQ(matrix * point.xyzw, test);
}

TEST(test_raymath, test_matrix3x3_submatrix) {
  auto matrix = raymath::Matrix<3, 3>(
    0., 0., 0.,
    4., 5., 0.,
    7., 8., 0.
  );

  auto test = raymath::Matrix<2, 2>(4., 5., 7., 8.);
  EXPECT_EQ(matrix.submatrix(0, 2), test);
}

TEST(test_raymath, test_matrix4x4_submatrix) {
  auto matrix = raymath::Matrix<4, 4>(
    1., 2., 0., 3.,
    4., 5., 0., 6.,
    7., 8., 0., 9.,
    0., 0., 0., 0.
  );

  auto test = raymath::Matrix<3, 3>(1., 2., 3., 4., 5., 6., 7., 8., 9.);
  EXPECT_EQ(matrix.submatrix(3, 2), test);
}