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

auto add(long long a, long long b) {
    return a + b;
}

auto greet(std::string name) {
    eppx_print(std::string("Hello, ") + name);
    return 0;
}

int main() {
    long long x = 5LL;    long long y = 10LL;    auto z = x + y * 2LL;    eppx_print(z);
    if (z > 20LL) {
    eppx_print(std::string("z is large"));
    } else {
    eppx_print(std::string("z is small"));
    }
    long long count = 0LL;    while (count < 3LL) {
    eppx_print(count);
    count = count + 1LL;
    }
    long long i;
    for (auto i_val : eppx_range(3LL)) {
        i = i_val;
    eppx_print(i);
    }
    auto result = add(7LL, 8LL);    eppx_print(result);
    greet(std::string("E++"));
    for (auto i_val : eppx_range(5LL)) {
        i = i_val;
    if (i == 2LL) {
    ; // pass statement
    } else if (i == 3LL) {
    continue;
    } else if (i == 4LL) {
    break;
    }
    eppx_print(i);
    }
    return 0;
}
