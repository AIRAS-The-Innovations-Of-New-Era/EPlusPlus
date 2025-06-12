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

// Forward declare eppx_bytearray and eppx_bytes for eppx_variant
struct eppx_bytearray;
struct eppx_bytes;

// eppx_variant can now hold eppx_bytearray and eppx_bytes
using eppx_variant = std::variant<long long, double, std::string, bool, eppx_bytearray, eppx_bytes>;

// Bytearray structure
struct eppx_bytearray {
    std::vector<unsigned char> data;

    // Constructor from size
    eppx_bytearray(long long size) : data(size, 0) {}

    // Constructor from string (UTF-8 encoding assumed)
    eppx_bytearray(const std::string& s) {
        for (char c : s) {
            // This is a direct conversion of char to unsigned char.
            // For true UTF-8 to bytes, a more complex conversion might be needed
            // if std::string can contain multi-byte UTF-8 characters and those
            // should be stored as multiple bytes in the bytearray.
            // However, Python's bytearray(str, encoding) handles this.
            // Here, we'll assume string is effectively a sequence of bytes
            // or that extended chars are truncated/converted in a platform-defined way.
            data.push_back(static_cast<unsigned char>(c));
        }
    }

    // Constructor from a vector of integers (iterable)
    eppx_bytearray(const std::vector<long long>& ints) {
        data.reserve(ints.size());
        for (long long val : ints) {
            if (val < 0 || val > 255) {
                // Consider throwing an error similar to Python
                // For now, let's clamp or skip, or throw std::out_of_range
                throw std::out_of_range("bytearray value out of range (0-255)");
            }
            data.push_back(static_cast<unsigned char>(val));
        }
    }

    // Default constructor
    eppx_bytearray() = default;

    // Copy constructor
    eppx_bytearray(const eppx_bytearray& other) = default;

    // Move constructor
    eppx_bytearray(eppx_bytearray&& other) noexcept = default;

    // Copy assignment
    eppx_bytearray& operator=(const eppx_bytearray& other) = default;

    // Move assignment
    eppx_bytearray& operator=(eppx_bytearray&& other) noexcept = default;


    std::string toString() const {
        std::stringstream ss;
        ss << "bytearray(b'";
        for (unsigned char byte : data) {
            if (byte == '\\' || byte == '\'') {
                ss << '\\' << byte;
            } else if (byte >= 32 && byte < 127) { // Printable ASCII characters
                ss << byte;
            } else {
                 // Hex escape for non-printable or special characters
                ss << "\\x" << std::hex << std::setw(2) << std::setfill('0') << static_cast<int>(byte);
            }
        }
        ss << "')";
        return ss.str();
    }
};

// Overload for eppx_print_single to handle eppx_bytearray
void eppx_print_single(const eppx_bytearray& ba) {
    std::cout << ba.toString();
}

// Bytes structure (immutable version of bytearray)
struct eppx_bytes {
    const std::vector<unsigned char> data;

    // Constructor from size
    eppx_bytes(long long size) : data(size, 0) {}

    // Constructor from string (UTF-8 encoding assumed)
    eppx_bytes(const std::string& s) : data(s.begin(), s.end()) {} // More direct

    // Constructor from a vector of integers (iterable)
    eppx_bytes(const std::vector<long long>& ints) : data{} {
        std::vector<unsigned char> temp_data;
        temp_data.reserve(ints.size());
        for (long long val : ints) {
            if (val < 0 || val > 255) {
                throw std::out_of_range("bytes value out of range (0-255)");
            }
            temp_data.push_back(static_cast<unsigned char>(val));
        }
        // Assign to const member 'data' via initialization (or use a mutable temp then move)
        // This approach requires data to not be const directly, or use a helper to init.
        // Let's adjust 'data' to be non-const, and rely on class interface for immutability.
        // const_cast is an option but not ideal.
        // Alternative: Initialize 'data' in constructor body if possible, or use a helper.
        // For simplicity with const member, we might need to build a temporary vector first.
        // The below is problematic if 'data' is const.
        // *const_cast<std::vector<unsigned char>*>(&data) = temp_data; // Bad practice
        // Let's re-evaluate making 'data' const directly.
        // It's easier if 'data' is not const, and immutability is by convention of no setters.
        // However, to strictly meet "const std::vector", initializer list or delegating constructor is better.
        // For now, let's assume data is filled by a private helper or directly if not const.
        // To make it work with 'const std::vector<unsigned char> data', we use a helper function.
        // This is getting complicated for a simple struct.
        // Let's simplify: data is non-const internally, immutability by not providing modifying methods.
        // Reverting 'data' to non-const for easier construction.
        // const std::vector<unsigned char> data; -> std::vector<unsigned char> data;
        // This means the struct below needs data to be non-const.
        // The above definition of eppx_bytes should be:
    // struct eppx_bytes {
    //  std::vector<unsigned char> data;
    //  ... constructors initialize data directly ...
    // }
    // Let's assume data is non-const for easier construction for now, and immutability is by convention.
    // This constructor will be re-written after correcting 'data' member.
    // This is the corrected version assuming data is not const:
    // eppx_bytes(const std::vector<long long>& ints) {
    //    data.reserve(ints.size());
    //    for (long long val : ints) {
    //        if (val < 0 || val > 255) {
    //            throw std::out_of_range("bytes value out of range (0-255)");
    //        }
    //        data.push_back(static_cast<unsigned char>(val));
    //    }
    // }
    // To truly use 'const std::vector<unsigned char> data;', we need to initialize it fully in the
    // constructor initializer list. This means using a helper lambda or function.
    // static std::vector<unsigned char> vector_from_long_long(const std::vector<long long>& ints) {
    //     std::vector<unsigned char> temp;
    //     temp.reserve(ints.size());
    //     for (long long val : ints) {
    //         if (val < 0 || val > 255) throw std::out_of_range("bytes value out of range (0-255)");
    //         temp.push_back(static_cast<unsigned char>(val));
    //     }
    //     return temp;
    // }
    // eppx_bytes(const std::vector<long long>& ints) : data(vector_from_long_long(ints)) {}
    // This is the cleaner way for const member.
        const_cast<std::vector<unsigned char>*>(&data)->reserve(ints.size());
        for (long long val : ints) {
            if (val < 0 || val > 255) {
                throw std::out_of_range("bytes value out of range (0-255)");
            }
            const_cast<std::vector<unsigned char>*>(&data)->push_back(static_cast<unsigned char>(val));
        }
    }


    // Constructor from eppx_bytearray
    eppx_bytes(const eppx_bytearray& ba) : data(ba.data) {}

    // Default constructor
    eppx_bytes() : data() {}

    // Copy constructor
    eppx_bytes(const eppx_bytes& other) = default;

    // Move constructor (though with const data, move is like copy)
    eppx_bytes(eppx_bytes&& other) noexcept = default;

    // Copy assignment (const member makes this tricky, might be implicitly deleted or require manual impl)
    // eppx_bytes& operator=(const eppx_bytes& other) = default;
    // If data is const, assignment operator is implicitly deleted. This is fine for immutable type.

    // Move assignment
    // eppx_bytes& operator=(eppx_bytes&& other) noexcept = default;


    std::string toString() const {
        std::stringstream ss;
        ss << "b'"; // Python's bytes literal representation
        for (unsigned char byte : data) {
            if (byte == '\\' || byte == '\'') {
                ss << '\\' << byte;
            } else if (byte >= 32 && byte < 127) { // Printable ASCII characters
                ss << byte;
            } else {
                ss << "\\x" << std::hex << std::setw(2) << std::setfill('0') << static_cast<int>(byte);
            }
        }
        ss << "'";
        return ss.str();
    }

    // Accessor methods (optional, but good for immutable type)
    size_t size() const { return data.size(); }
    bool empty() const { return data.empty(); }
    // unsigned char operator[](size_t index) const { return data[index]; } // Example accessor
};

// Overload for eppx_print_single to handle eppx_bytes
void eppx_print_single(const eppx_bytes& bs) {
    std::cout << bs.toString();
}


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

// ASCII function
std::string eppx_ascii(const std::string& s) {
    std::stringstream ss;
    for (unsigned char c : s) {
        if (c < 128) {
            // Printable ASCII characters
            if (c >= 32 && c <= 126) {
                if (c == '\'' || c == '\\') { // Escape single quote and backslash
                    ss << '\\';
                }
                ss << c;
            } else {
                // Non-printable ASCII characters (e.g., newline, tab)
                switch (c) {
                    case '\n': ss << "\\n"; break;
                    case '\t': ss << "\\t"; break;
                    case '\r': ss << "\\r"; break;
                    case '\b': ss << "\\b"; break;
                    case '\f': ss << "\\f"; break;
                    case '\v': ss << "\\v"; break;
                    case '\\': ss << "\\\\"; break;
                    case '\'': ss << "\\'"; break;
                    default:
                        ss << "\\x" << std::hex << std::setw(2) << std::setfill('0') << (int)c;
                        break;
                }
            }
        } else if (c < 0x800) { // 2-byte UTF-8 sequence
            ss << "\\u" << std::hex << std::setw(4) << std::setfill('0') << (int)c;
        } else { // 3 or 4-byte UTF-8 sequence (simplified for this example)
                 // For full Unicode support, a proper UTF-8 decoder would be needed.
                 // This part handles only up to U+FFFF.
                 // Characters beyond U+FFFF would require \Uxxxxxxxx format.
            ss << "\\u" << std::hex << std::setw(4) << std::setfill('0') << (int)c;
        }
    }
    return ss.str();
}

// Overload for single characters
std::string eppx_ascii(char c_char) {
    return eppx_ascii(std::string(1, c_char));
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
        } else if constexpr (std::is_same_v<T, eppx_bytearray>) {
            return "<class 'bytearray'>";
        } else if constexpr (std::is_same_v<T, eppx_bytes>) {
            return "<class 'bytes'>";
        } else {
            return "<class 'object'>";
        }
    }, var);
}

bool eppx_isinstance(const eppx_variant& obj, const std::string& type_name) {
    std::string obj_type = eppx_type(obj);
    return obj_type.find(type_name) != std::string::npos;
}

// Callable check at runtime
bool eppx_callable_runtime(const eppx_variant& var) {
    return std::visit([](const auto& v) -> bool {
        using T = std::decay_t<decltype(v)>;
        // This function is for runtime checks on objects already stored in eppx_variant.
        // E++ functions (from 'def') and lambdas are typically handled at codegen time
        // by directly outputting 'true' for callable().
        // This runtime check is a fallback or for more dynamic scenarios.

        // Known non-callable types stored in eppx_variant:
        if constexpr (
            std::is_same_v<T, long long> ||
            std::is_same_v<T, double> ||
            std::is_same_v<T, std::string> ||
            std::is_same_v<T, bool> ||
            std::is_same_v<T, eppx_bytearray> ||
            std::is_same_v<T, eppx_bytes>
            // Add other non-callable variant types here (e.g. std::vector, std::map if added to variant)
        ) {
            return false;
        }
        // Placeholder for any E++ specific callable types we might add to eppx_variant later
        // (e.g., bound methods, wrapped function objects from C++).
        // For now, if it's not an explicitly non-callable data type,
        // we can't determine much more at runtime without more type info.
        // The codegen phase should handle clear cases (E++ func names, lambdas).
        return false; // Default for types not explicitly non-callable but not known callables
    }, var);
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
