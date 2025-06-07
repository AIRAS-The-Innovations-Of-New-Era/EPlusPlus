#include <iostream>
#include <string>
#include <cmath> // Added for std::pow

void eppx_print(const std::string& s) { std::cout << s << std::endl; }
void eppx_print(int x) { std::cout << x << std::endl; }

int main() {
    int x = 42;
    eppx_print(x);
    eppx_print(123);
    auto y = (x + 5);
    eppx_print(y);
    eppx_print((1 + (2 * 3)));
    eppx_print("yo welcome from e++");
    eppx_print((10 % 3));
    eppx_print(static_cast<long long>(std::pow(2, 4)));
    eppx_print((10 / 3));
    eppx_print((11 / 3));
    eppx_print((static_cast<long long>(std::pow(5, 2)) % 3));
    eppx_print((2 * static_cast<long long>(std::pow(3, 2))));
    eppx_print((10 == 10));
    eppx_print((10 == 5));
    eppx_print((10 != 5));
    eppx_print((10 != 10));
    eppx_print((10 > 5));
    eppx_print((5 > 10));
    eppx_print((10 < 20));
    eppx_print((20 < 10));
    eppx_print((10 >= 10));
    eppx_print((10 >= 11));
    eppx_print((10 <= 10));
    eppx_print((10 <= 9));
    eppx_print((x == 42));
    eppx_print((y != (x + 5)));
    return 0;
}
