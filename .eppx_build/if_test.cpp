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
    long long x;
    x = 10LL;
    long long y;
    y = 20LL;
    long long z;
    z = 30LL;
    if ((x > y)) {
    eppx_print(std::string("x > y"));
    } else if ((x == y)) {
    eppx_print(std::string("x == y"));
    } else {
    eppx_print(std::string("x < y"));
    }
    if ((z > y)) {
    eppx_print(std::string("z > y"));
    }
    if ((x == 10LL)) {
    eppx_print(std::string("x is ten"));
    } else if ((x == 11LL)) {
    eppx_print(std::string("x is eleven"));
    } else if ((x == 12LL)) {
    eppx_print(std::string("x is twelve"));
    } else {
    eppx_print(std::string("x is something else"));
    }
    eppx_print(std::string("If/elif/else tests complete."));
    return 0;
}
