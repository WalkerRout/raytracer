
#include <cmath>

#include "raymath.h"

namespace raymath {

// vector methods
auto Vector::zero(void) -> Vector {
  return Vector({0, 0, 0});
}

// vector + vector = vector
auto Vector::operator+(Vector rhs) const -> Vector {
  rhs.xyzw[0] += xyzw[0];
  rhs.xyzw[1] += xyzw[1];
  rhs.xyzw[2] += xyzw[2];
  rhs.xyzw[3] += xyzw[3];
  return rhs;
}

// vector + point = point
auto Vector::operator+(Point rhs) const -> Point {
  rhs.xyzw[0] += xyzw[0];
  rhs.xyzw[1] += xyzw[1];
  rhs.xyzw[2] += xyzw[2];
  rhs.xyzw[3] += xyzw[3];
  return rhs;
}

auto Vector::operator-(void) const -> Vector {
  return Vector({-xyzw[0], -xyzw[1], -xyzw[2]});
}

auto Vector::operator-(Vector rhs) const -> Vector {
  auto vector = Vector();
  vector.xyzw = xyzw;
  vector.xyzw[0] -= rhs.xyzw[0];
  vector.xyzw[1] -= rhs.xyzw[1];
  vector.xyzw[2] -= rhs.xyzw[2];
  vector.xyzw[3] -= rhs.xyzw[3];
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
  point.xyzw[0] += xyzw[0];
  point.xyzw[1] += xyzw[1];
  point.xyzw[2] += xyzw[2];
  point.xyzw[3] += xyzw[3];
  return point;
}

auto Point::operator-(void) const -> Point {
  return Point({-xyzw[0], -xyzw[1], -xyzw[2]});
}

auto Point::operator-(Point rhs) const -> Vector {
  auto vector = Vector();
  vector.xyzw = xyzw;
  vector.xyzw[0] -= rhs.xyzw[0];
  vector.xyzw[1] -= rhs.xyzw[1];
  vector.xyzw[2] -= rhs.xyzw[2];
  vector.xyzw[3] -= rhs.xyzw[3];
  return vector;
}

auto Point::operator-(Vector rhs) const -> Point {
  auto point = Point();
  point.xyzw = xyzw;
  point.xyzw[0] -= rhs.xyzw[0];
  point.xyzw[1] -= rhs.xyzw[1];
  point.xyzw[2] -= rhs.xyzw[2];
  point.xyzw[3] -= rhs.xyzw[3];
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

// matrix methods are in header for SFINAE resolution

}