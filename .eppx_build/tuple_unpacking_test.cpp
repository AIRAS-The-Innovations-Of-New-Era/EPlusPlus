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
#include <sstream> // For stringstream
#include <bitset>  // For bitset
#include <functional> // For std::hash

void eppx_print(const std::string& s) { std::cout << s << std::endl; }
void eppx_print(long long x) { std::cout << x << std::endl; }
void eppx_print(double x) { std::cout << x << std::endl; }
void eppx_print(bool b) { std::cout << (b ? "true" : "false") << std::endl; }
void eppx_print(const std::complex<long long>& c) { std::cout << "(" << c.real() << (c.imag() >= 0 ? "+" : "") << c.imag() << "j)" << std::endl; }
void eppx_print(const std::complex<double>& c) { std::cout << "(" << c.real() << (c.imag() >= 0 ? "+" : "") << c.imag() << "j)" << std::endl; }
void eppx_print(std::nullptr_t) { std::cout << "None" << std::endl; }
// Basic type printing functions
void eppx_print_single(bool b) { std::cout << (b ? "true" : "false"); }
void eppx_print_single(char c) { std::cout << c; }
void eppx_print_single(signed char x) { std::cout << static_cast<int>(x); }
void eppx_print_single(unsigned char x) { std::cout << static_cast<unsigned int>(x); }
void eppx_print_single(short x) { std::cout << x; }
void eppx_print_single(unsigned short x) { std::cout << x; }
void eppx_print_single(int x) { std::cout << x; }
void eppx_print_single(unsigned int x) { std::cout << x; }
void eppx_print_single(long x) { std::cout << x; }
void eppx_print_single(unsigned long x) { std::cout << x; }
void eppx_print_single(long long x) { std::cout << x; }
void eppx_print_single(unsigned long long x) { std::cout << x; }
void eppx_print_single(float x) { std::cout << x; }
void eppx_print_single(double x) { std::cout << x; }
void eppx_print_single(long double x) { std::cout << x; }
void eppx_print_single(const std::string& s) { std::cout << s; }
void eppx_print_single(const char* s) { std::cout << s; }
void eppx_print_single(std::nullptr_t) { std::cout << "None"; }
template<typename T> void eppx_print_single(const std::vector<T>& vec) {
    std::cout << "[";
    for (size_t i = 0; i < vec.size(); ++i) {
        if (i > 0) std::cout << ", ";
        std::cout << vec[i];
    }
    std::cout << "]";
}
template<typename K, typename V> void eppx_print_single(const std::map<K, V>& m) {
    std::cout << "{";
    bool first = true;
    for (const auto& pair : m) {
        if (!first) std::cout << ", ";
        std::cout << pair.first << ": " << pair.second;
        first = false;
    }
    std::cout << "}";
}
template<typename T> void eppx_print_single(const std::set<T>& s) {
    std::cout << "{";
    bool first = true;
    for (const auto& item : s) {
        if (!first) std::cout << ", ";
        std::cout << item;
        first = false;
    }
    std::cout << "}";
}
template<typename T, typename... Args> void eppx_print(T&& first, Args&&... args) {
    eppx_print_single(first);
    if constexpr (sizeof...(args) > 0) {
        std::cout << " ";
        eppx_print(args...);
    } else {
        std::cout << std::endl;
    }
}

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

template<typename T> auto eppx_abs(T x) { return x < 0 ? -x : x; }
template<typename T> auto eppx_pow(T base, T exp) { return static_cast<T>(std::pow(base, exp)); }
template<typename T> auto eppx_round(T x) { return static_cast<long long>(std::round(x)); }
template<typename T> std::string eppx_hex(T x) { std::stringstream ss; ss << "0x" << std::hex << x; return ss.str(); }
template<typename T> std::string eppx_bin(T x) { return "0b" + std::bitset<64>(x).to_string().substr(std::bitset<64>(x).to_string().find('1')); }
template<typename T> std::string eppx_oct(T x) { std::stringstream ss; ss << "0o" << std::oct << x; return ss.str(); }
template<typename T> auto eppx_chr(T x) { return std::string(1, static_cast<char>(x)); }
auto eppx_ord(const std::string& s) { return s.empty() ? 0LL : static_cast<long long>(s[0]); }

template<typename T> auto eppx_int(T x) { return static_cast<long long>(x); }
auto eppx_int(const std::string& s) { return std::stoll(s); }
template<typename T> auto eppx_float(T x) { return static_cast<double>(x); }
auto eppx_float(const std::string& s) { return std::stod(s); }
template<typename T> auto eppx_bool(T x) { return static_cast<bool>(x); }
template<typename T> auto eppx_str(T x) { std::stringstream ss; ss << x; return ss.str(); }
auto eppx_str(const std::string& s) { return s; }
auto eppx_str(std::nullptr_t) { return std::string("None"); }

template<typename T> auto eppx_len(const std::vector<T>& v) { return static_cast<long long>(v.size()); }
template<typename T> auto eppx_len(const std::set<T>& s) { return static_cast<long long>(s.size()); }
template<typename K, typename V> auto eppx_len(const std::map<K, V>& m) { return static_cast<long long>(m.size()); }
auto eppx_len(const std::string& s) { return static_cast<long long>(s.length()); }
template<typename... Args> auto eppx_len(const std::tuple<Args...>& t) { return static_cast<long long>(sizeof...(Args)); }

template<typename T> auto eppx_min(const std::vector<T>& v) { return v.empty() ? T{} : *std::min_element(v.begin(), v.end()); }
template<typename T> auto eppx_max(const std::vector<T>& v) { return v.empty() ? T{} : *std::max_element(v.begin(), v.end()); }
template<typename T, typename... Args> auto eppx_min(T first, Args... args) { return std::min({first, static_cast<T>(args)...}); }
template<typename T, typename... Args> auto eppx_max(T first, Args... args) { return std::max({first, static_cast<T>(args)...}); }

template<typename T> auto eppx_sum(const std::vector<T>& v) { T result = T{}; for (const auto& x : v) result += x; return result; }
template<typename T> auto eppx_sum(const std::vector<T>& v, T start) { T result = start; for (const auto& x : v) result += x; return result; }

template<typename T> bool eppx_all(const std::vector<T>& v) { for (const auto& x : v) if (!x) return false; return true; }
template<typename T> bool eppx_any(const std::vector<T>& v) { for (const auto& x : v) if (x) return true; return false; }

template<typename T> auto eppx_sorted(const std::vector<T>& v) { auto result = v; std::sort(result.begin(), result.end()); return result; }
template<typename T> auto eppx_reversed(const std::vector<T>& v) { auto result = v; std::reverse(result.begin(), result.end()); return result; }
auto eppx_reversed(const std::string& s) { auto result = s; std::reverse(result.begin(), result.end()); return result; }

template<typename T> auto eppx_enumerate(const std::vector<T>& v) {
    std::vector<std::tuple<long long, T>> result;
    for (size_t i = 0; i < v.size(); ++i) {
        result.emplace_back(static_cast<long long>(i), v[i]);
    }
    return result;
}

template<typename T1, typename T2> auto eppx_zip(const std::vector<T1>& v1, const std::vector<T2>& v2) {
    std::vector<std::tuple<T1, T2>> result;
    size_t min_size = std::min(v1.size(), v2.size());
    for (size_t i = 0; i < min_size; ++i) {
        result.emplace_back(v1[i], v2[i]);
    }
    return result;
}

template<typename F, typename T> auto eppx_map(F func, const std::vector<T>& v) {
    std::vector<decltype(func(v[0]))> result;
    for (const auto& x : v) {
        result.push_back(func(x));
    }
    return result;
}

template<typename F, typename T> auto eppx_filter(F func, const std::vector<T>& v) {
    std::vector<T> result;
    for (const auto& x : v) {
        if (func(x)) {
            result.push_back(x);
        }
    }
    return result;
}

template<typename T> auto eppx_list(const std::vector<T>& v) { return v; }
auto eppx_list() { return std::vector<long long>{}; }
template<typename... Args> auto eppx_tuple(Args... args) { return std::make_tuple(args...); }
template<typename T> auto eppx_set(const std::vector<T>& v) { return std::set<T>(v.begin(), v.end()); }
auto eppx_set() { return std::set<long long>{}; }
template<typename K, typename V> auto eppx_dict(const std::vector<std::tuple<K, V>>& pairs) {
    std::map<K, V> result;
    for (const auto& p : pairs) {
        result[std::get<0>(p)] = std::get<1>(p);
    }
    return result;
}
auto eppx_dict() { return std::map<std::string, long long>{}; }

template<typename T> bool eppx_isinstance(T, const std::string& type_name) {
    // Simplified type checking - would need proper runtime type info
    return false; // Placeholder
}

template<typename T> std::string eppx_type(T) {
    // Simplified type info - would need proper runtime type info
    return "<type>"; // Placeholder
}

std::string eppx_input(const std::string& prompt = "") {
    if (!prompt.empty()) std::cout << prompt;
    std::string result;
    std::getline(std::cin, result);
    return result;
}

template<typename T> std::unordered_set<T> eppx_internal_make_frozenset(const std::vector<T>& initial_elements) { return std::unordered_set<T>(initial_elements.begin(), initial_elements.end()); }

int main() {
    return 0;
}
