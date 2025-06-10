#ifndef EPPX_BUILTINS_HPP
#define EPPX_BUILTINS_HPP

#include <string>
#include <vector>
#include <map>
#include <set>
#include <iostream>
#include <sstream>
#include <algorithm>
#include <numeric>
#include <variant>
#include <functional>
#include <type_traits>
#include <iomanip>

// Forward declarations for eppx_variant (simplified for now)
using eppx_variant = std::variant<long long, double, std::string, bool>;

// Range function (already exists)
std::vector<long long> eppx_range(long long n) {
    std::vector<long long> result;
    result.reserve(n);
    for (long long i = 0; i < n; ++i) {
        result.push_back(i);
    }
    return result;
}

// String representation functions
std::string eppx_hex(long long n) {
    std::stringstream ss;
    ss << "0x" << std::hex << n;
    return ss.str();
}

std::string eppx_bin(long long n) {
    if (n == 0) return "0b0";
    
    std::string binary = "";
    long long temp = std::abs(n);
    while (temp > 0) {
        binary = (temp % 2 == 0 ? "0" : "1") + binary;
        temp /= 2;
    }
    return "0b" + (n < 0 ? "-" : "") + binary;
}

std::string eppx_oct(long long n) {
    std::stringstream ss;
    ss << "0o" << std::oct << n;
    return ss.str();
}

// Collection functions
template<typename Container>
auto eppx_sum(const Container& container) -> typename Container::value_type {
    return std::accumulate(container.begin(), container.end(), typename Container::value_type{});
}

template<typename Container>
bool eppx_all(const Container& container) {
    return std::all_of(container.begin(), container.end(), 
                      [](const auto& item) { return static_cast<bool>(item); });
}

template<typename Container>
bool eppx_any(const Container& container) {
    return std::any_of(container.begin(), container.end(), 
                      [](const auto& item) { return static_cast<bool>(item); });
}

template<typename Container>
Container eppx_reversed(Container container) {
    std::reverse(container.begin(), container.end());
    return container;
}

template<typename Container>
Container eppx_sorted(Container container) {
    std::sort(container.begin(), container.end());
    return container;
}

// Collection converters
template<typename T>
std::vector<T> eppx_to_list(const std::vector<T>& vec) {
    return vec; // Already a list
}

template<typename T>
std::vector<T> eppx_to_list(const std::set<T>& s) {
    return std::vector<T>(s.begin(), s.end());
}

template<typename T>
std::set<T> eppx_to_set(const std::vector<T>& vec) {
    return std::set<T>(vec.begin(), vec.end());
}

// I/O functions
std::string eppx_input() {
    std::string line;
    std::getline(std::cin, line);
    return line;
}

std::string eppx_input(const std::string& prompt) {
    std::cout << prompt;
    std::string line;
    std::getline(std::cin, line);
    return line;
}

// Type functions
std::string eppx_type(const eppx_variant& var) {
    return std::visit([](const auto& v) -> std::string {
        using T = std::decay_t<decltype(v)>;
        if constexpr (std::is_same_v<T, long long>) {
            return "<class 'int'>";
        } else if constexpr (std::is_same_v<T, double>) {
            return "<class 'float'>";
        } else if constexpr (std::is_same_v<T, std::string>) {
            return "<class 'str'>";
        } else if constexpr (std::is_same_v<T, bool>) {
            return "<class 'bool'>";
        } else {
            return "<class 'object'>";
        }
    }, var);
}

bool eppx_isinstance(const eppx_variant& obj, const std::string& type_name) {
    std::string obj_type = eppx_type(obj);
    return obj_type.find(type_name) != std::string::npos;
}

bool eppx_callable(const eppx_variant& obj) {
    // Simplified: for now, assume only functions are callable
    // This would need more sophisticated type system in real implementation
    return false;
}

// Object attribute functions (simplified stubs)
bool eppx_hasattr(const eppx_variant& obj, const std::string& name) {
    // Stub implementation - would need object system
    return false;
}

eppx_variant eppx_getattr(const eppx_variant& obj, const std::string& name) {
    // Stub implementation - would need object system
    throw std::runtime_error("getattr not implemented for this type");
}

eppx_variant eppx_getattr(const eppx_variant& obj, const std::string& name, const eppx_variant& default_value) {
    // Stub implementation - would need object system
    return default_value;
}

void eppx_setattr(const eppx_variant& obj, const std::string& name, const eppx_variant& value) {
    // Stub implementation - would need object system
    throw std::runtime_error("setattr not implemented for this type");
}

void eppx_delattr(const eppx_variant& obj, const std::string& name) {
    // Stub implementation - would need object system
    throw std::runtime_error("delattr not implemented for this type");
}

// Higher-order functions (simplified implementations)
template<typename Container>
std::vector<std::pair<size_t, typename Container::value_type>> eppx_enumerate(const Container& container) {
    std::vector<std::pair<size_t, typename Container::value_type>> result;
    size_t i = 0;
    for (const auto& item : container) {
        result.emplace_back(i++, item);
    }
    return result;
}

template<typename... Containers>
auto eppx_zip(const Containers&... containers) {
    // Simplified zip implementation for 2 containers
    // Full implementation would need variadic template handling
    std::vector<std::tuple<typename Containers::value_type...>> result;
    // This is a placeholder - full zip implementation is complex
    return result;
}

template<typename Func, typename Container>
auto eppx_map(Func func, const Container& container) {
    std::vector<decltype(func(*container.begin()))> result;
    std::transform(container.begin(), container.end(), std::back_inserter(result), func);
    return result;
}

template<typename Func, typename Container>
auto eppx_filter(Func func, const Container& container) {
    Container result;
    std::copy_if(container.begin(), container.end(), std::back_inserter(result), func);
    return result;
}

#endif // EPPX_BUILTINS_HPP
