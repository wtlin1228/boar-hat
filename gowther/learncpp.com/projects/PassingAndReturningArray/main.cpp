#include <array>
#include <iostream>

template <typename T, auto N> void passByRef(const std::array<T, N> &arr) {
  static_assert(N != 0);
  std::cout << arr[0] << '\n';

  std::cout << std::get<3>(arr) << '\n';
}

// can be used when the program isn't performance-sensitive
template <typename T, auto N> std::array<T, N> returnByValue() {
  std::array<T, N> arr{};
  return arr;
}

int main() {
  std::array arr{9, 7, 5, 3, 1};

  passByRef(arr);

  return 0;
}
