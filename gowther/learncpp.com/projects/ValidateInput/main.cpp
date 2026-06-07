#include <cassert>
#include <iostream>
#include <limits>
#include <stdexcept>
#include <string>

void ignoreLine() {
  std::cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');
}

// return true if extraction failed, false otherwise
bool clearFailedExtraction() {
  if (!std::cin) {
    if (std::cin.eof()) {
      std::exit(0);
    }

    std::cin.clear();
    ignoreLine();

    return true;
  }

  return false;
}

// returns true if std::cin has unextracted input on the current line, false
// otherwise
bool hasUnextractedInput() {
  if (!std::cin.eof() && std::cin.peek() != '\n') {
    ignoreLine();
    return true;
  }

  return false;
}

double getDouble() {
  while (true) {
    std::cout << "Enter a decimal number: ";
    double x{};
    std::cin >> x;

    if (clearFailedExtraction() || hasUnextractedInput()) {
      continue;
    }

    return x;
  }
}

char getOperator() {
  while (true) {
    std::cout << "Enter one of the following: +, -, *, or /: ";
    char operation{};
    std::cin >> operation;

    if (clearFailedExtraction() || hasUnextractedInput()) {
      continue;
    }

    switch (operation) {
    case '+':
    case '-':
    case '*':
    case '/':
      return operation;
    default:
      std::cout << "Oops, that input is invalid. Please try again.\n";
    }
  }
}

void printResult(double x, char operation, double y) {
  switch (operation) {
  case '+':
    std::cout << x << ' ' << operation << ' ' << y << " is " << x + y << '\n';
    return;
  case '-':
    std::cout << x << ' ' << operation << ' ' << y << " is " << x - y << '\n';
    return;
  case '*':
    std::cout << x << ' ' << operation << ' ' << y << " is " << x * y << '\n';
    return;
  case '/':
    assert(y != 0.0 && "Could not divide by zero");
    if (y == 0.0) {
      throw std::runtime_error("Could not divide by zero");
    }
    std::cout << x << ' ' << operation << ' ' << y << " is " << x / y << '\n';
    return;
  default:
    throw std::runtime_error(std::string("Invalid operator: ") + operation);
  }
}

int main() {
  try {
    printResult(getDouble(), getOperator(), getDouble());
  } catch (const std::runtime_error &e) {
    std::cerr << "Error: " << e.what() << '\n';
    return 1;
  }

  return 0;
}
