#include <array>
#include <iostream>

template <typename T, std::size_t Row, std::size_t Col>
using Array2d = std::array<std::array<T, Col>, Row>;

template <typename T, std::size_t Row, std::size_t Col>
constexpr int rowLength(const Array2d<T, Row, Col> &) {
  return Row;
}

template <typename T, std::size_t Row, std::size_t Col>
constexpr int colLength(const Array2d<T, Row, Col> &) {
  return Col;
}

template <typename T, std::size_t Row, std::size_t Col>
void printArray(const Array2d<T, Row, Col> &arr) {
  for (const auto &row : arr) {
    for (const auto &e : row) {
      std::cout << e << '\t';
    }
    std::cout << '\n';
  }
}

int main() {
  Array2d<int, 3, 4> arr{{
      {1, 2, 3, 4},
      {11, 12, 13, 14},
      {21, 22, 23, 24},
  }};

  std::cout << "Rows: " << rowLength(arr) << '\n';
  std::cout << "Cols: " << colLength(arr) << '\n';

  printArray(arr);

  return 0;
}
