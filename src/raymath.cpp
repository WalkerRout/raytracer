
#include <cmath>

#include "raymath.h"

namespace raymath {

// vector methods
auto Vector::zero(void) -> Vector {
  return Vector({0., 0., 0.});
}

// vector + vector = vector
auto Vector::operator+(Vector rhs) const -> Vector {
  const auto& ref = xyzw.get_data();
  auto& rhs_ref = rhs.xyzw.get_data_ref();

  rhs_ref[0] += ref[0];
  rhs_ref[1] += ref[1];
  rhs_ref[2] += ref[2];
  rhs_ref[3] += ref[3];

  return rhs;
}

// vector + point = point
auto Vector::operator+(Point rhs) const -> Point {
  const auto& ref = xyzw.get_data();
  auto& rhs_ref = rhs.xyzw.get_data_ref();

  rhs_ref[0] += ref[0];
  rhs_ref[1] += ref[1];
  rhs_ref[2] += ref[2];
  rhs_ref[3] += ref[3];

  return rhs;
}

auto Vector::operator-(void) const -> Vector {
  return Vector({-xyzw[0], -xyzw[1], -xyzw[2]});
}

auto Vector::operator-(Vector rhs) const -> Vector {
  auto vector = Vector();
  vector.xyzw = xyzw;

  auto& res_ref = vector.xyzw.get_data_ref();
  auto& rhs_ref = rhs.xyzw.get_data_ref();

  res_ref[0] -= rhs_ref[0];
  res_ref[1] -= rhs_ref[1];
  res_ref[2] -= rhs_ref[2];
  res_ref[3] -= rhs_ref[3];

  return vector;
}

auto Vector::operator*(double scalar) const -> Vector {
  return Vector({xyzw[0] * scalar, xyzw[1] * scalar, xyzw[2] * scalar});
}

auto Vector::operator/(double scalar) const -> Vector {
  return Vector({xyzw[0] / scalar, xyzw[1] / scalar, xyzw[2] / scalar});
}

auto Vector::operator==(const Vector rhs) const -> bool {
  auto result = true;

  result &= rhs.xyzw[0] == xyzw[0];
  result &= rhs.xyzw[1] == xyzw[1];
  result &= rhs.xyzw[2] == xyzw[2];
  result &= rhs.xyzw[3] == xyzw[3];

  return result;
}

auto operator<<(std::ostream& os, Vector vector) -> std::ostream& {
  return os 
    << "V(" 
    << vector.xyzw[0] << ", " 
    << vector.xyzw[1] << ", " 
    << vector.xyzw[2] 
    << ")";
}

auto Vector::set(const size_t i, const double x) -> void {
  assert(i < 4 && "Index out of bounds for vector");
  xyzw.get_data_ref()[i] = x;
}
auto Vector::get(const size_t i) const -> double {
  assert(i < 4 && "Index out of bounds for vector");
  return xyzw.get_data()[i];
}

auto Vector::get_ref(const size_t i) -> double& {
  assert(i < 4 && "Index out of bounds for vector");
  return xyzw.get_data_ref()[i];
}

auto Vector::dot(Vector rhs) const -> double {
  return (
    xyzw[0] * rhs.xyzw[0] +
    xyzw[1] * rhs.xyzw[1] +
    xyzw[2] * rhs.xyzw[2] +
    xyzw[3] * rhs.xyzw[3]
  );
}

auto Vector::cross(Vector rhs) const -> Vector {
  return Vector({
    xyzw[1] * rhs.xyzw[2] - xyzw[2] * rhs.xyzw[1],
    xyzw[2] * rhs.xyzw[0] - xyzw[0] * rhs.xyzw[2],
    xyzw[0] * rhs.xyzw[1] - xyzw[1] * rhs.xyzw[0]
  });
}

auto Vector::magnitude(void) const -> double {
  return std::sqrt(
    std::pow(xyzw[0], 2) + 
    std::pow(xyzw[1], 2) + 
    std::pow(xyzw[2], 2) + 
    std::pow(xyzw[3], 2)
  );
}

auto Vector::normalize(void) const -> Vector {
  auto magnitude = this->magnitude();
  auto vector = Vector();
  vector.xyzw = {
    xyzw[0] / magnitude, 
    xyzw[1] / magnitude, 
    xyzw[2] / magnitude, 
    xyzw[3] / magnitude
  };

  return vector;
}

// point methods
auto Point::origin(void) -> Point {
  return Point({0, 0, 0});
}

// point + vector = point
auto Point::operator+(Vector rhs) const -> Point {
  auto point = Point();
  point.xyzw = std::move(rhs.xyzw);

  const auto& ref = xyzw.get_data();
  auto& res_ref = point.xyzw.get_data_ref();

  res_ref[0] += ref[0];
  res_ref[1] += ref[1];
  res_ref[2] += ref[2];
  res_ref[3] += ref[3];

  return point;
}

auto Point::operator-(void) const -> Point {
  return Point({-xyzw[0], -xyzw[1], -xyzw[2]});
}

auto Point::operator-(Point rhs) const -> Vector {
  auto vector = Vector();
  vector.xyzw = xyzw;

  const auto& rhs_ref = rhs.xyzw.get_data();
  auto& res_ref = vector.xyzw.get_data_ref();

  res_ref[0] -= rhs_ref[0];
  res_ref[1] -= rhs_ref[1];
  res_ref[2] -= rhs_ref[2];
  res_ref[3] -= rhs_ref[3];

  return vector;
}

auto Point::operator-(Vector rhs) const -> Point {
  auto point = Point();
  point.xyzw = xyzw;

  const auto& rhs_ref = rhs.xyzw.get_data();
  auto& res_ref = point.xyzw.get_data_ref();

  res_ref[0] -= rhs_ref[0];
  res_ref[1] -= rhs_ref[1];
  res_ref[2] -= rhs_ref[2];
  res_ref[3] -= rhs_ref[3];

  return point;
}

auto Point::operator*(double scalar) const -> Point {
  return Point({xyzw[0] * scalar, xyzw[1] * scalar, xyzw[2] * scalar});
}

auto Point::operator/(double scalar) const -> Point {
  return Point({xyzw[0] / scalar, xyzw[1] / scalar, xyzw[2] / scalar});
}

auto Point::operator==(const Point rhs) const -> bool {
  auto result = true;

  result &= rhs.xyzw[0] == xyzw[0];
  result &= rhs.xyzw[1] == xyzw[1];
  result &= rhs.xyzw[2] == xyzw[2];
  result &= rhs.xyzw[3] == xyzw[3];

  return result;
}

auto operator<<(std::ostream& os, Point point) -> std::ostream& {
  return os 
    << "P(" 
    << point.xyzw[0] << ", " 
    << point.xyzw[1] << ", " 
    << point.xyzw[2] 
    << ")";
}

auto Point::set(const size_t i, const double x) -> void {
  assert(i < 4 && "Index out of bounds for point");
  xyzw.get_data_ref()[i] = x;
}
auto Point::get(const size_t i) const -> double {
  assert(i < 4 && "Index out of bounds for point");
  return xyzw.get_data()[i];
}

auto Point::get_ref(const size_t i) -> double& {
  assert(i < 4 && "Index out of bounds for point");
  return xyzw.get_data_ref()[i];
}

// matrix methods are in header for SFINAE resolution

}