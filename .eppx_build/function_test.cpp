#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <cmath> // Added for std::pow

void eppx_print(const std::string& s) { std::cout << s << std::endl; }
void eppx_print(long long x) { std::cout << x << std::endl; }
void eppx_print(double x) { std::cout << x << std::endl; }
void eppx_print(bool b) { std::cout << (b ? "true" : "false") << std::endl; }

std::vector<long long> eppx_range(long long n) {
    std::vector<long long> result;
    for (long long i = 0; i < n; ++i) {
        result.push_back(i);
    }
    return result;
}

auto add(auto a, auto b) {
    auto result = (a + b);
    eppx_print(std::string("In add, result is:"));
    eppx_print(result);
    return result;
}

auto greet(auto name) {
    eppx_print(std::string("Hello, "));
    eppx_print(name);
    return 0;
}

int main() {
    eppx_print(std::string("Testing function call:"));
    auto sum = add(3LL, 4LL);
    eppx_print(std::string("Sum is:"));
    eppx_print(sum);
    greet(std::string("E++ user!"));
    eppx_print(std::string("Function test complete!"));
    return 0;
}
