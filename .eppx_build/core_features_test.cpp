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

template<typename T0, typename T1>
auto add(T0 a, T1 b) {
    return a + b;
}

template<typename T0>
auto greet(T0 name) {
    eppx_print(std::string("Hello, ") + name);
    return 0; // Default return if none explicit
}

int main() {
    long long x = 5LL;
    long long y = 10LL;
    auto z = x + y * 2LL;
    eppx_print(z);
    if (z > 20LL) {
        eppx_print(std::string("z is large"));
    } else {
        eppx_print(std::string("z is small"));
    }
    long long count = 0LL;
    while (count < 3LL) {
        eppx_print(count);
        count = count + 1LL;
    }
    for (auto i : eppx_range(3LL)) {
            eppx_print(i);
    }
    auto result = add(7LL, 8LL);
    eppx_print(result);
    greet(std::string("E++"));
    for (auto i : eppx_range(5LL)) {
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
