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
    eppx_print(std::string("Testing basic for loop:"));
    long long i;
    for (auto i_val : eppx_range(5LL)) {
        i = i_val;
    eppx_print(i);
    }
    eppx_print(std::string("Testing for loop with assignment inside:"));
    long long total;
    total = 0LL;
    long long num;
    for (auto num_val : eppx_range(10LL)) {
        num = num_val;
    total += num;
    }
    eppx_print(total);
    eppx_print(std::string("Testing nested for loops:"));
    for (auto i_val : eppx_range(3LL)) {
        i = i_val;
    long long j;
    for (auto j_val : eppx_range(2LL)) {
        j = j_val;
    double result;
    result = ((i * 10LL) + j);
    eppx_print(result);
    }
    }
    eppx_print(std::string("For loop test completed!"));
    return 0;
}
