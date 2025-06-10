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
    long long x = 42LL;
    eppx_print(x);
    eppx_print(123LL);
    auto y = x + 5LL;
    eppx_print(y);
    eppx_print(1LL + 2LL * 3LL);
    eppx_print(std::string("yo welcome from e++"));
    eppx_print(10LL % 3LL);
    eppx_print(static_cast<long long>(std::pow(2LL, 4LL)));
    eppx_print(10LL / 3LL);
    eppx_print(11LL / 3LL);
    eppx_print(static_cast<long long>(std::pow(5LL, 2LL)) % 3LL);
    eppx_print(2LL * static_cast<long long>(std::pow(3LL, 2LL)));
    eppx_print(10LL == 10LL);
    eppx_print(10LL == 5LL);
    eppx_print(10LL != 5LL);
    eppx_print(10LL != 10LL);
    eppx_print(10LL > 5LL);
    eppx_print(5LL > 10LL);
    eppx_print(10LL < 20LL);
    eppx_print(20LL < 10LL);
    eppx_print(10LL >= 10LL);
    eppx_print(10LL >= 11LL);
    eppx_print(10LL <= 10LL);
    eppx_print(10LL <= 9LL);
    eppx_print(x == 42LL);
    eppx_print(y != x + 5LL);
    long long a = 10LL;
    eppx_print(a);
    a += 5LL;
    eppx_print(a);
    a -= 3LL;
    eppx_print(a);
    a *= 2LL;
    eppx_print(a);
    a /= 4LL;
    eppx_print(a);
    long long b = 25LL;
    b %= 4LL;
    eppx_print(b);
    long long c = 2LL;
    c = static_cast<long long>(std::pow(static_cast<double>(c), static_cast<double>(3LL)));
    eppx_print(c);
    long long d = 17LL;
    d = static_cast<long long>(std::floor(static_cast<double>(d) / static_cast<double>(5LL)));
    eppx_print(d);
    eppx_print(1LL && 0LL);
    eppx_print(1LL && 1LL);
    eppx_print(0LL && 0LL);
    eppx_print(1LL || 0LL);
    eppx_print(0LL || 1LL);
    eppx_print(0LL || 0LL);
    eppx_print(!(1LL));
    eppx_print(!(0LL));
    eppx_print(x == 42LL && y == 47LL);
    eppx_print(x == 42LL || y == 0LL);
    eppx_print(!(x == 0LL));
    eppx_print(6LL & 3LL);
    eppx_print(6LL | 3LL);
    eppx_print(6LL ^ 3LL);
    eppx_print(~(6LL));
    eppx_print(3LL << 2LL);
    eppx_print(12LL >> 2LL);
    long long val1 = 100LL;
    long long val2 = 100LL;
    eppx_print(val1 == val2 /* Placeholder for IS */);
    eppx_print(val1 != val2 /* Placeholder for IS NOT */);
    std::string str_a = std::string("hello world");
    std::string str_b = std::string("world");
    std::string str_c = std::string("earth");
    eppx_print(str_a.find(str_b) != std::string::npos /* Placeholder for IN */);
    eppx_print(str_a.find(str_c) != std::string::npos /* Placeholder for IN */);
    eppx_print(str_a.find(str_b) == std::string::npos /* Placeholder for NOT IN */);
    eppx_print(str_a.find(str_c) == std::string::npos /* Placeholder for NOT IN */);
    long long bit_val = 6LL;
    bit_val &= 3LL;
    eppx_print(bit_val);
    bit_val = 6LL;
    bit_val |= 3LL;
    eppx_print(bit_val);
    bit_val = 6LL;
    bit_val ^= 3LL;
    eppx_print(bit_val);
    bit_val = 3LL;
    bit_val <<= 2LL;
    eppx_print(bit_val);
    bit_val = 12LL;
    bit_val >>= 2LL;
    eppx_print(bit_val);
    eppx_print(std::string("Operator tests complete."));
    return 0;
}
