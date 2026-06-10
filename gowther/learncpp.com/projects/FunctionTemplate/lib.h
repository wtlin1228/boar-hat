#ifndef LIB_H
#define LIB_H

#include <iostream>

template <typename T> void printIdAndValue(T value) {
  static int id{0};
  std::cout << ++id << ") " << value << '\n';
}

#endif