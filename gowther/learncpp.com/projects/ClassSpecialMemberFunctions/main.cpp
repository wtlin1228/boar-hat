#include <iostream>

class Simple {
private:
  int m_id{};

public:
  Simple(int id) : m_id{id} {
    std::cout << "Constructing Simple " << m_id << '\n';
  }

  Simple(const Simple &other) : m_id{other.m_id + 1} {
    std::cout << "Copying Simple " << other.m_id << " to Simple " << m_id
              << '\n';
  }

  ~Simple() { std::cout << "Destructing Simple " << m_id << '\n'; }
};

int main() {
  Simple simple1{1};
  {
    Simple simple2{simple1};
  } // simple2 dies here

  return 0;
} // simple1 dies here
