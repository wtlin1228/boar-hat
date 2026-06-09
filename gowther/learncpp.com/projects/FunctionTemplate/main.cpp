#include "lib.h"
#include <iostream>

void foo();

template <typename T> T addOne(T x) { return x + 1; }

// Use function template specialization to tell the compiler that
// addOne(const char*) should emit a compilation error
// template <> const char *addOne(const char *x) = delete;

int main() {
  // uncomment line 10 to see the compilation error
  std::cout << addOne("Hello, world!") << '\n';

  // printIdAndValue is defined in lib.h, so every translation unit that calls
  // it (main.cpp and foo.cpp here) instantiates its own identical copy of
  // printIdAndValue<const char*>. The compiler marks these as weak/COMDAT
  // symbols; the linker keeps just one and discards the rest. That's why the
  // duplicates don't trigger a "multiple definition" (ODR) error.
  printIdAndValue("main");
  foo();

  return 0;
}
