#ifndef raymath_H
#define raymath_H

#include <algorithm>
#include <iostream>
#include <cassert>
#include <cstring>
//#include <limits>
#include <memory>
#include <array>
#include <cmath>

// used to determine whether or not to store matrix on stack or heap
#ifndef STACK_LOC_CUTOFF
#define STACK_LOC_CUTOFF (128*128)
#endif

namespace raymath {

// function forward definitions
template <typename T>
constexpr auto eqf(const T a, const T b, const T epsilon) -> bool;

template <size_t N, size_t M>
struct Matrix {
  using storage_t = std::array<double, N*M>;
  using data_t = std::conditional<N*M >= STACK_LOC_CUTOFF, std::unique_ptr<storage_t>, storage_t>::type;

  Matrix() {
    if constexpr (N*M >= STACK_LOC_CUTOFF)
      data = std::make_unique<storage_t>();
    memset((void*)get_data_ref().data(), 0, sizeof get_data());
  }

  template <typename... Args>
  Matrix(Args&& ...args) {
    if constexpr (N*M >= STACK_LOC_CUTOFF)
      data = std::make_unique<storage_t>((storage_t){args...});
    else
      data = storage_t{args...};
  }

  // Rule of Zero (though no copy operations because of std::unique_ptr)

  static auto identity(void) -> Matrix requires (N == M);

  auto get_rows() const -> size_t;
  auto get_cols() const -> size_t;
  auto get_data() const -> const storage_t&;
  auto get_data_ref() -> storage_t&;
  auto set(const size_t i, const size_t j, const double x) -> void;
  auto get(const size_t i, const size_t j) const -> double;
  auto get_ref(const size_t i, const size_t j) -> double&;

  auto operator+(const Matrix& rhs) const -> Matrix;
  auto operator-(const Matrix& rhs) const -> Matrix;
  auto operator/(const Matrix& rhs) const -> Matrix;
  auto hadamard(const Matrix& rhs) const -> Matrix;
  template <size_t P, size_t Q> 
  auto operator==(const Matrix<P, Q>& rhs) const -> bool requires (N != P && M != Q);
  auto operator==(const Matrix& rhs) const -> bool;
  auto operator[](const size_t i) const -> double;

  template <size_t P, size_t Q>
  friend auto operator<<(std::ostream& os, const Matrix<P, Q>& mat) -> std::ostream&;

  template <size_t P, size_t Q> // * (dot product)
  auto operator*(const Matrix<P, Q>& rhs) const -> Matrix<N, Q> requires (M == P);
  auto transpose() const -> Matrix<M, N>;

  auto on_heap() const -> bool;
private:
  data_t data;
};

class Point;
struct Vector {
  Vector() = default;
  Vector(std::array<double, 3> il) {
    auto& ref = xyzw.get_data_ref();
    ref[0] = il[0];
    ref[1] = il[1];
    ref[2] = il[2];
    ref[3] = 0.0;
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

  auto set(const size_t i, const double x) -> void;
  auto get(const size_t i) const -> double;
  auto get_ref(const size_t i) -> double&;

  auto dot(Vector rhs) const -> double;
  auto cross(Vector rhs) const -> Vector;
  auto magnitude(void) const -> double;
  auto normalize(void) const -> Vector;

public:
  Matrix<4, 1> xyzw;
};

struct Point {
  Point() = default;
  Point(std::array<double, 3> il) {
    auto& ref = xyzw.get_data_ref();
    ref[0] = il[0];
    ref[1] = il[1];
    ref[2] = il[2];
    ref[3] = 1.0;
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

  auto set(const size_t i, const double x) -> void;
  auto get(const size_t i) const -> double;
  auto get_ref(const size_t i) -> double&;

public:
  Matrix<4, 1> xyzw;
};

template <size_t N, size_t M>
auto Matrix<N, M>::identity(void) -> Matrix requires (N == M) {
  auto result = Matrix<N, M>();

  for(int i = 0; i < N; ++i)
    result.set(i, i, 1.);

  return result;
}

template <size_t N, size_t M>
auto Matrix<N, M>::get_rows() const -> size_t { return N; }

template <size_t N, size_t M>
auto Matrix<N, M>::get_cols() const -> size_t { return M; }

#define GET_DATA_COMMON_BODY \
  do {  \
    if constexpr (N*M >= STACK_LOC_CUTOFF) { \
      return *data; \
    } else { \
      return data; \
    } \
  } while(0)

template <size_t N, size_t M>
auto Matrix<N, M>::get_data() const -> const Matrix::storage_t& { GET_DATA_COMMON_BODY; }

template <size_t N, size_t M>
auto Matrix<N, M>::get_data_ref() -> Matrix::storage_t& { GET_DATA_COMMON_BODY; }

template <size_t N, size_t M>
auto Matrix<N, M>::get(const size_t i, const size_t j) const -> double {
  assert(i < N && j < M && "Indices i and j out of bounds for matrix");
  return get_data()[i * M + j];
}

template <size_t N, size_t M>
auto Matrix<N, M>::get_ref(const size_t i, const size_t j) -> double& {
  assert(i < N && j < M && "Indices i and j out of bounds for matrix");
  return get_data_ref()[i * M + j];
}

template <size_t N, size_t M>
auto Matrix<N, M>::set(const size_t i, const size_t j, double x) -> void {
  assert(i < N && j < M && "Indices i and j out of bounds for matrix");
  get_data_ref()[i * M + j] = x;
}

template <size_t N, size_t M>
auto Matrix<N, M>::operator+(const Matrix& rhs) const -> Matrix {
  auto result = Matrix<N, M>();
  auto data = get_data();
  auto rhs_data = rhs.get_data();

  std::transform(data.cbegin(), data.cend(), rhs_data.cbegin(),
                 result.get_data_ref().begin(), std::plus<double>());

  return result;
}

template <size_t N, size_t M>
auto Matrix<N, M>::operator-(const Matrix& rhs) const -> Matrix {
  auto result = Matrix<N, M>();
  auto data = get_data();
  auto rhs_data = rhs.get_data();

  std::transform(data.cbegin(), data.cend(), rhs_data.cbegin(),
                 result.get_data_ref().begin(), std::minus<double>());
  
  return result;
}

template <size_t N, size_t M>
auto Matrix<N, M>::operator/(const Matrix& rhs) const -> Matrix {
  auto result = Matrix<N, M>();
  auto data = get_data();
  auto rhs_data = rhs.get_data();

  std::transform(data.cbegin(), data.cend(), rhs_data.cbegin(),
                 result.get_data_ref().begin(), std::divides<double>());
  
  return result;
}

template <size_t N, size_t M>
auto Matrix<N, M>::hadamard(const Matrix& rhs) const -> Matrix {
  auto result = Matrix<N, M>();
  auto data = get_data();
  auto rhs_data = rhs.get_data();

  std::transform(data.cbegin(), data.cend(), rhs_data.cbegin(),
                 result.get_data_ref().begin(), std::multiplies<double>());
  
  return result;
}

template <size_t N, size_t M>
template <size_t P, size_t Q>
auto Matrix<N, M>::operator==(const Matrix<P, Q>& rhs) const -> bool requires (N != P && M != Q) { return false; }

template <size_t N, size_t M>
auto Matrix<N, M>::operator==(const Matrix& rhs) const -> bool {
  // you have no idea how much i want to do this... goddamn floats
  // memcmp((unsigned char *)get_data().data(), (unsigned char *)rhs.get_data().data(), sizeof get_data());
  assert(get_data().size() == rhs.get_data().size());
  auto data = get_data();
  auto rhs_data = rhs.get_data();

  return std::equal(data.cbegin(), data.cend(), rhs_data.cbegin(), [](const auto& a, const auto& b) {
    // std::numeric_limits<double>::epsilon() 
    return std::abs(a - b) <= pow(0.1, 40); 
  });
}

template <size_t N, size_t M>
auto operator<<(std::ostream& os, const Matrix<N, M>& mat) -> std::ostream& {
  for(int i = 0; i < N; ++i) {
    for(int j = 0; j < M; ++j) {
      os << mat.get_data()[i * M + j];
      os << ' ';
    }
    os << '\n';
  }
  return os;
}

template <size_t N, size_t M>
auto Matrix<N, M>::operator[](const size_t i) const -> double {
  assert(i < N*M && "Index out of bounds for matrix");
  return get_data()[i];
}

template <size_t N, size_t M>
template <size_t P, size_t Q>
auto Matrix<N, M>::operator*(const Matrix<P, Q>& rhs) const -> Matrix<N, Q> requires (M == P) {
  auto result = Matrix<N, Q>();

  for(int i = 0; i < N; ++i)
    for(int j = 0; j < Q; ++j)
      for(int k = 0; k < P; ++k)
        result.get_ref(i, j) += get(i, k) * rhs.get(k, j);

  return result;
}

template <size_t N, size_t M>
auto Matrix<N, M>::transpose() const -> Matrix<M, N> {
  auto result = Matrix<M, N>();

  if constexpr (N == M) {
    // this is based around the assumption that memcpy does not require void *src to be writeable...
    memcpy(result.get_data_ref().data(), get_data().data(), sizeof get_data());
    for(int i = 0; i < N; ++i)
      for(int j = i + 1; j < M; ++j)
        std::swap(result.get_ref(j, i), result.get_ref(i, j));
  } else {
    for(int i = 0; i < N; ++i)
      for(int j = 0; j < M; ++j)
        result.set(j, i, get(i, j));
  }

  return result;
}

template <size_t N, size_t M>
auto Matrix<N, M>::on_heap() const -> bool {
  return std::is_same<Matrix::data_t, std::unique_ptr<Matrix::storage_t>>::value;
}

template <typename T>
constexpr auto eqf(const T a, const T b, const T epsilon) -> bool {
  return std::abs(a - b) <= (std::max(a, b) * epsilon);
}

}

#endif // raymath_H