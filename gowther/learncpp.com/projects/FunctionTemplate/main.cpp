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

  // In modern C++, "inline" means "multiple definitions allowed": such a
  // function may be defined in many translation units without violating the
  // ODR, and the linker collapses the copies into one. Function templates are
  // implicitly treated as inline functions, so instantiations get this same
  // treatment.
  //
  // main.cpp and foo.cpp both instantiate printIdAndValue<const char*> from
  // lib.h. The linker keeps one copy, so all callers share its single static
  // `id` -- notice the counter keeps climbing across calls instead of resetting.
  printIdAndValue("main");
  foo();

  return 0;
}
