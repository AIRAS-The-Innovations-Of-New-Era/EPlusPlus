#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <cmath> // For std::pow
#include <complex> // For std::complex
#include <tuple>   // For std::tuple
#include <map>     // For std::map
#include <set>     // For std::set
#include <unordered_set> // For std::unordered_set

void eppx_print(const std::string& s) { std::cout << s << std::endl; }
void eppx_print(long long x) { std::cout << x << std::endl; }
void eppx_print(double x) { std::cout << x << std::endl; }
void eppx_print(bool b) { std::cout << (b ? "true" : "false") << std::endl; }
void eppx_print(const std::complex<long long>& c) { std::cout << "(" << c.real() << (c.imag() >= 0 ? "+" : "") << c.imag() << "j)" << std::endl; }
void eppx_print(const std::complex<double>& c) { std::cout << "(" << c.real() << (c.imag() >= 0 ? "+" : "") << c.imag() << "j)" << std::endl; }
void eppx_print(std::nullptr_t) { std::cout << "None" << std::endl; }
template<typename T> void eppx_print(const std::vector<T>& vec) { std::cout << "list object (size: " << vec.size() << ")" << std::endl; }
template<typename K, typename V> void eppx_print(const std::map<K, V>& m) { std::cout << "dict object (size: " << m.size() << ")" << std::endl; }
template<typename T> void eppx_print(const std::set<T>& s) { std::cout << "set object (size: " << s.size() << ")" << std::endl; }
template<typename T> void eppx_print(const std::unordered_set<T>& s) { std::cout << "frozenset object (size: " << s.size() << ")" << std::endl; }
template <typename... Args> void eppx_print(const std::tuple<Args...>& t) { std::cout << "tuple object (size: " << sizeof...(Args) << ")" << std::endl; }

std::vector<long long> eppx_range(long long n) {
    std::vector<long long> result;
    for (long long i = 0; i < n; ++i) {
        result.push_back(i);
    }
    return result;
}

template<typename T> std::unordered_set<T> eppx_internal_make_frozenset(const std::vector<T>& initial_elements) { return std::unordered_set<T>(initial_elements.begin(), initial_elements.end()); }

int main() {
    eppx_print(std::string("Testing basic for loop:"));
    for (auto i : eppx_range(5LL)) {
            eppx_print(i);
    }
    eppx_print(std::string("Testing for loop with assignment inside:"));
    long long total = 0LL;
    for (auto num : eppx_range(10LL)) {
            total += num;
    }
    eppx_print(total);
    eppx_print(std::string("Testing nested for loops:"));
    for (auto i : eppx_range(3LL)) {
            for (auto j : eppx_range(2LL)) {
                    auto result = i * 10LL + j;
                    eppx_print(result);
            }
    }
    eppx_print(std::string("For loop test completed!"));
    return 0;
}
