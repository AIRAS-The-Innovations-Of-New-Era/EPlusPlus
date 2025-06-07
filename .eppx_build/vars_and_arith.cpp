#include <iostream>
#include <string>

void eppx_print(const std::string& s) { std::cout << s << std::endl; }
void eppx_print(int x) { std::cout << x << std::endl; }

int main() {
    int x = 42;
    eppx_print(x);
    eppx_print(123);
    auto y = x;
    eppx_print(y);
    eppx_print(1);
    return 0;
}
