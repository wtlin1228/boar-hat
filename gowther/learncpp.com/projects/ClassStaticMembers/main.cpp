#include <iostream>

class Something {
private:
  // use inline so we can initialize this static member variable inside the
  // class body
  static inline int s_idGenerator{1};

  int m_id{};

public:
  Something() : m_id{s_idGenerator++} {}

  static int getNextID() { return s_idGenerator; }

  int getID() const { return m_id; }
};

int main() {
  Something first{};
  Something second{};
  Something third{};

  std::cout << first.getID() << '\n';
  std::cout << second.getID() << '\n';
  std::cout << third.getID() << '\n';

  std::cout << "next id: " << Something::getNextID() << '\n';

  return 0;
}
