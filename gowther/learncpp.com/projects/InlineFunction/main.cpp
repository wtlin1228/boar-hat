#include "libs/pi.h"
#include <iostream>

double circumference(double radius);

int main() {
  std::cout << pi() << '\n';
  std::cout << circumference(2.0) << '\n';

  return 0;
}
