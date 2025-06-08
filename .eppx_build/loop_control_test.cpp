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

int main() {
    long long i = 0LL;
    while (i < 10LL) {
    i += 1LL;
    if (i == 3LL) {
    ; // pass statement
    }
    if (i == 5LL) {
    continue;
    }
    if (i == 8LL) {
    break;
    }
    eppx_print(i);
    }
    eppx_print(std::string("done"));
    return 0;
}
