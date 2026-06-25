#include <cassert>
#include <iostream>
#include <vector>

namespace Students {
enum Names : unsigned int // explicitly specifies the underlying type is
                          // unsigned int
{
  kenny,   // 0
  kyle,    // 1
  stan,    // 2
  butters, // 3
  cartman, // 4
  // add future enumerators here
  max_students // 5, count enumerator
};
}

void printClass(const std::vector<int> &testScores) {
  assert(std::size(testScores) == Students::max_students);

  std::cout << "The class has " << Students::max_students << " students\n";

  std::cout << testScores[Students::kenny] << '\n';
  std::cout << testScores[Students::kyle] << '\n';
  std::cout << testScores[Students::stan] << '\n';
  std::cout << testScores[Students::butters] << '\n';
  std::cout << testScores[Students::cartman] << '\n';
}

int main() {
  std::vector<int> testScores(Students::max_students);

  Students::Names name{Students::stan}; // non-constexpr
  testScores[name] = 76; // not a sign conversion since name is unsigned

  printClass(testScores);

  return 0;
}