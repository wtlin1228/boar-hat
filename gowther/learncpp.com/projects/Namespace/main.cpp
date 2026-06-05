#include "foo.h"
#include "goo.h"

#include <iostream>

int doSomething(int x, int y);

int main() {
  namespace Foo = Qoo::Foo;
  namespace Goo = Qoo::Goo;

  std::cout << Foo::doSomething(42, 10) << '\n';
  std::cout << Goo::doSomething(42, 10) << '\n';

  return 0;
}
