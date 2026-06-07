#include "foo.h"
#include "goo.h"

#include <iostream>

int main() {
  // namespace alias
  namespace Foo = Qoo::Foo;
  namespace Goo = Qoo::Goo;
  std::cout << Foo::doSomething(10, 7) << '\n';
  std::cout << Goo::doSomething(10, 7) << '\n';

  // namespace using declaration
  {
    using Foo::doSomething;
    std::cout << doSomething(10, 7) << '\n';
  }
  {
    using Goo::doSomething;
    std::cout << doSomething(10, 7) << '\n';
  }

  // namespace using directive (avoid using it)
  {
    using namespace Foo;
    std::cout << doSomething(10, 7) << '\n';
  }
  {
    using namespace Goo;
    std::cout << doSomething(10, 7) << '\n';
  }

  return 0;
}
