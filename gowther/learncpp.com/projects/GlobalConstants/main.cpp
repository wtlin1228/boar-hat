#include "libs/constants.h"
#include <iostream>

int main() {
  std::cout << "The circumference of 0.5 is: " << 2 * 0.5 * constants::pi
            << '\n';

  std::cout << "RGB(" << constants::colorR << ',' << constants::colorG << ','
            << constants::colorB << ")\n";

  return 0;
}
