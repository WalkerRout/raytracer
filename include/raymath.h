#ifndef raymath_H
#define raymath_H

#include <iostream>
#include <variant>
#include <cassert>
#include <memory>
#include <array>

#define STACK_LOC_CUTOFF (16*256)

namespace raymath {

class Point;
struct Vector {
  Vector() = default;
  Vector(std::array<double, 3> il) {
    xyzw[0] = il[0];
    xyzw[1] = il[1];
    xyzw[2] = il[2];
    xyzw[3] = 0.0;
  }

  // Rule of Zero

  static auto zero(void) -> Vector;

  auto operator+(Vector rhs) const -> Vector;
  auto operator+(Point rhs) const -> Point;
  auto operator-(void) const -> Vector;
  auto operator-(Vector rhs) const -> Vector;
  auto operator*(double scalar) const -> Vector;
  auto operator/(double scalar) const -> Vector;
  auto operator==(const Vector rhs) const -> bool;

  friend auto operator<<(std::ostream& os, Vector vector) -> std::ostream&;

  auto dot(Vector rhs) const -> double;
  auto cross(Vector rhs) const -> Vector;
  auto magnitude(void) const -> double;
  auto normalize(void) const -> Vector;

public:
  std::array<double, 4> xyzw{0, 0, 0, 0};
};

struct Point {
  Point() = default;
  Point(std::array<double, 3> il) {
    xyzw[0] = il[0];
    xyzw[1] = il[1];
    xyzw[2] = il[2];
    xyzw[3] = 1.0;
  }

  // Rule of Zero

  static auto origin(void) -> Point;

  auto operator+(Vector rhs) const -> Point;
  auto operator-(void) const -> Point;
  auto operator-(Point rhs) const -> Vector;
  auto operator-(Vector rhs) const -> Point;
  auto operator*(double scalar) const -> Point;
  auto operator/(double scalar) const -> Point;
  auto operator==(const Point rhs) const -> bool;

  friend auto operator<<(std::ostream& os, Point point) -> std::ostream&;

public:
  std::array<double, 4> xyzw{0, 0, 0, 1};
};

// rows, cols
// i   , j
// y   , x
template <size_t N, size_t M>
struct Matrix {
  using storage_t = std::array<double, N*M>;
  using storage_ptr = std::unique_ptr<storage_t>;

  static const bool what = is_trivially_copyable<T>::value;
  typedef typename std::conditional<what, storage_t, storage_ptr>::type data_type;

  Matrix() = default;
  template <typename... Args>
  Matrix(Args&& ...args) {
    if constexpr (N*M >= STACK_LOC_CUTOFF) {
      data = std::unique_ptr<storage_t>({args...});
    } else {
      data = std::array<double, N*M>{args...};
    }
  }

  auto get_rows() const -> size_t;
  auto get_cols() const -> size_t;
  auto get_data() const -> const storage_t&;
  auto set(size_t i, size_t j, double x);
  auto get(size_t i, size_t j) const -> double;

  auto operator+(Matrix& rhs) const -> Matrix;
  auto operator-(Matrix& rhs) const -> Matrix;
  auto operator/(Matrix& rhs) const -> Matrix;
  auto operator*(Matrix& rhs) const -> Matrix;

  template <size_t TN, size_t TM>
  friend auto operator<<(std::ostream& os, const Matrix<TN, TM>& mat) -> std::ostream&;

public:
  size_t rows{N};
  size_t cols{M};
private:
  data_t data;
};

template <size_t N, size_t M>
auto Matrix<N, M>::get_rows() const -> size_t { return N; }

template <size_t N, size_t M>
auto Matrix<N, M>::get_cols() const -> size_t { return M; }

template <size_t N, size_t M>
auto Matrix<N, M>::get_data() const -> const Matrix::storage_t& {
  if(std::holds_alternative<Matrix::storage_t>(data)) {
    return std::get<Matrix::storage_t>(data);
  } else {
    return *std::get<std::unique_ptr<Matrix::storage_t>>(data);
  }
}

// y = i, x = j
template <size_t N, size_t M>
auto Matrix<N, M>::get(size_t i, size_t j) const -> double {
  assert(i < N && j < M && "Indices i and j out of bounds for matrix");
  return get_data()[i * M + j];
}

template <size_t N, size_t M>
auto Matrix<N, M>::set(size_t i, size_t j, double x) {
  assert(i < N && j < M && "Indices i and j out of bounds for matrix");
  data[i * M + j] = x;
}

template <size_t N, size_t M>
auto Matrix<N, M>::operator+(Matrix& rhs) const -> Matrix { return Matrix<N, M>(); }

template <size_t N, size_t M>
auto Matrix<N, M>::operator-(Matrix& rhs) const -> Matrix { return Matrix<N, M>(); }

template <size_t N, size_t M>
auto Matrix<N, M>::operator/(Matrix& rhs) const -> Matrix { return Matrix<N, M>(); }

template <size_t N, size_t M>
auto Matrix<N, M>::operator*(Matrix& rhs) const -> Matrix {
  auto result = Matrix<M, rhs.rows()>();

  for(int i = 0; i < N; ++i) {

  }


 /*
 for (int i = 0; i < a.rows; ++i)
    for (int j = 0; j < b.cols; ++j)
      for (int k = 0; k < b.rows; ++k)
        *mat_get_ptr(&c, i, j) += mat_get(a, i, k) * mat_get(b, k, j);
  */

  return result;
}

template <size_t N, size_t M>
auto operator<<(std::ostream& os, const Matrix<N, M>& mat) -> std::ostream& {
  for(int i = 0; i < N; ++i) {
    for(int j = 0; j < M; ++j) {
      os << mat.data[i * M + j];
      os << ' ';
    }
    os << '\n';
  }
  return os;
}

}

#endif // raymath_H