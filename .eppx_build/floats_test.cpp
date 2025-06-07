#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <cmath> // Added for std::pow

void eppx_print(const std::string& s) { std::cout << s << std::endl; }
void eppx_print(long long x) { std::cout << x << std::endl; }
void eppx_print(double x) { std::cout << x << std::endl; }
void eppx_print(int x) { std::cout << x << std::endl; }
void eppx_print(bool b) { std::cout << (b ? "true" : "false") << std::endl; }

int main() {
    double x;
    x = 3.14;
    eppx_print(x);
    double y;
    y = 2.5;
    eppx_print(y);
    double z;
    z = (x + y);
    eppx_print(z);
    double result;
    result = (x * y);
    eppx_print(result);
    double division;
    division = (x / y);
    eppx_print(division);
    double mixed;
    mixed = (x + 10LL);
    eppx_print(mixed);
    eppx_print((x > y));
    eppx_print((x < y));
    eppx_print((x == 3.14));
    x += 1.5;
    eppx_print(x);
    y *= 2;
    eppx_print(y);
    eppx_print(std::string("Float tests complete."));
    return 0;
}
