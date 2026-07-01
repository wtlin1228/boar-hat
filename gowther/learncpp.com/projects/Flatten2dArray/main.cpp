#include <array>
#include <functional>
#include <iostream>

template <typename T, std::size_t Row, std::size_t Col>
using ArrayFlat2d = std::array<T, Row * Col>;

template <typename T, std::size_t Row, std::size_t Col> class ArrayView2d {
private:
  std::reference_wrapper<ArrayFlat2d<T, Row, Col>> m_arr{};

public:
  ArrayView2d(ArrayFlat2d<T, Row, Col> &arr) : m_arr{arr} {};

  // Get element via single subscript (using operator[])
  const T &operator[](int i) const {
    return m_arr.get()[static_cast<std::size_t>(i)];
  }
  T &operator[](int i) { return m_arr.get()[static_cast<std::size_t>(i)]; }

  // Get element via 2d subscript (using operator())
  const T &operator()(int row, int col) const {
    return m_arr.get()[static_cast<std::size_t>((row * cols()) + col)];
  }

  [[nodiscard]] int rows() const { return static_cast<int>(Row); }
  [[nodiscard]] int cols() const { return static_cast<int>(Col); }
  [[nodiscard]] int length() const { return static_cast<int>(Row * Col); }
};

int main() {
  ArrayFlat2d<int, 3, 4> arr{1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12};
  ArrayView2d<int, 3, 4> arrView{arr};

  std::cout << "Rows: " << arrView.rows() << '\n';
  std::cout << "Cols: " << arrView.cols() << '\n';

  for (int i = 0; i < arrView.length(); ++i) {
    std::cout << arrView[i] << ' ';
  }

  std::cout << '\n';

  for (int row = 0; row < arrView.rows(); ++row) {
    for (int col = 0; col < arrView.cols(); ++col) {
      std::cout << arrView(row, col) << '\t';
    }
    std::cout << '\n';
  }
  std::cout << '\n';

  return 0;
}
