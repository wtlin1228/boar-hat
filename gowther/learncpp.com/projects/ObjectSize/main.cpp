#include <climits> // for CHAR_BIT
#include <iomanip> // for std::setw (which sets the width of the subsequent output)
#include <iostream>

int main() {

  std::cout << "A byte is " << CHAR_BIT << " bits\n\n";

  std::cout << std::left; // left justify output

  constexpr int width{16};
  std::cout << std::setw(width) << "bool:" << sizeof(bool) << " bytes\n";
  std::cout << std::setw(width) << "char:" << sizeof(char) << " bytes\n";
  std::cout << std::setw(width) << "short:" << sizeof(short) << " bytes\n";
  std::cout << std::setw(width) << "int:" << sizeof(int) << " bytes\n";
  std::cout << std::setw(width) << "long:" << sizeof(long) << " bytes\n";
  std::cout << std::setw(width) << "long long:" << sizeof(long long)
            << " bytes\n";
  std::cout << std::setw(width) << "float:" << sizeof(float) << " bytes\n";
  std::cout << std::setw(width) << "double:" << sizeof(double) << " bytes\n";
  std::cout << std::setw(width) << "long double:" << sizeof(long double)
            << " bytes\n";

  return 0;
}