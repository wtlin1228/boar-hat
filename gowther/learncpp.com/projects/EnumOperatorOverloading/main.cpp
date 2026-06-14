#include <cstdint>
#include <ios>
#include <iostream>
#include <limits>
#include <optional>
#include <ostream>
#include <string_view>

enum Color : std::int8_t { unknown, black, white, red, green, blue };

constexpr std::string_view getColorName(Color color) {
  switch (color) {
  case black:
    return "black";
  case white:
    return "white";
  case red:
    return "red";
  case green:
    return "green";
  case blue:
    return "blue";
  default:
    return "unknown";
  }
}

constexpr std::optional<Color> getColorFromString(std::string_view sv) {
  if (sv == "black") {
    return black;
  }
  if (sv == "white") {
    return white;
  }
  if (sv == "red") {
    return red;
  }
  if (sv == "green") {
    return green;
  }
  if (sv == "blue") {
    return blue;
  }

  return {};
}

// Teach operator<< how to print a Color
std::ostream &operator<<(std::ostream &out, Color color) {
  out << getColorName(color);
  return out;
}

std::istream &operator>>(std::istream &in, Color &color) {
  std::string s{};
  in >> s;

  std::optional<Color> match{getColorFromString(s)};
  if (match) {
    color = *match;
    return in;
  }

  in.setstate(std::ios_base::failbit);

  return in;
}

int main() {
  std::cout << "Enter a color: black, white, red, green, or blue: ";

  Color shirt{};
  std::cin >> shirt;

  if (std::cin) {
    std::cout << "Your shirt is " << shirt << '\n';
  } else {
    std::cin.clear();
    std::cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');
    std::cout << "Your color was not valid\n";
  }

  return 0;
}
