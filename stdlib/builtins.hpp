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

// Forward declarations
struct eppx_bytearray;
struct eppx_bytes;
struct eppx_tuple;
struct eppx_frozenset;

// Define type aliases
using eppx_list_variant_t = std::vector<eppx_variant>;
using eppx_dict_variant_t = std::map<std::string, eppx_variant>;
using eppx_set_variant_t = std::set<eppx_variant>; // Requires operator< for eppx_variant

// eppx_variant definition
using eppx_variant = std::variant<
    std::nullptr_t,
    long long,
    double,
    std::string,
    bool,
    std::complex<double>,
    eppx_bytearray,
    eppx_bytes,
    eppx_list_variant_t,
    eppx_dict_variant_t,
    eppx_tuple,          // Added tuple
    eppx_frozenset,     // Added frozenset
    eppx_set_variant_t   // Added set (mutable set type)
>;

// Forward declare comparison for eppx_variant (needed for std::set)
bool operator<(const eppx_variant& lhs, const eppx_variant& rhs);
// Forward declare hash for eppx_variant (needed for frozenset elements if they are tuples/other frozensets)
long long eppx_hash(const eppx_variant& var);


// eppx_tuple structure
struct eppx_tuple {
    eppx_list_variant_t elements; // Internally use a vector like lists

    eppx_tuple() = default;
    explicit eppx_tuple(eppx_list_variant_t els) : elements(std::move(els)) {}

    bool operator==(const eppx_tuple& other) const; // Declaration
    bool operator<(const eppx_tuple& other) const;  // Declaration for ordering
    // Hash will be computed by eppx_hash_visitor
};

// eppx_frozenset structure
struct eppx_frozenset {
    eppx_set_variant_t data; // Store elements in a set to ensure uniqueness and order for hashing

    eppx_frozenset() = default;
    explicit eppx_frozenset(eppx_set_variant_t s) : data(std::move(s)) {}

    bool operator==(const eppx_frozenset& other) const; // Declaration
    bool operator<(const eppx_frozenset& other) const;  // Declaration for ordering
    // Hash will be computed by eppx_hash_visitor
};

bool eppx_tuple::operator==(const eppx_tuple& other) const {
    return elements == other.elements; // Relies on eppx_variant::operator==
}

bool eppx_tuple::operator<(const eppx_tuple& other) const {
    return std::lexicographical_compare(
        elements.begin(), elements.end(),
        other.elements.begin(), other.elements.end()
        // This defaults to using operator< for eppx_variant
    );
}

// eppx_frozenset structure
struct eppx_frozenset {
    eppx_set_variant_t data; // Store elements in a set to ensure uniqueness and order for hashing

    eppx_frozenset() = default;
    explicit eppx_frozenset(eppx_set_variant_t s) : data(std::move(s)) {}

    bool operator==(const eppx_frozenset& other) const {
        return data == other.data; // Relies on eppx_variant::operator== and operator< (for std::set)
    }
    bool operator<(const eppx_frozenset& other) const {
        // Lexicographical comparison of sorted elements
        return std::lexicographical_compare(
            data.begin(), data.end(),
            other.data.begin(), other.data.end()
        );
    }
    // Hash will be computed by eppx_hash_visitor
};


// operator< for eppx_variant
// Critical for std::set<eppx_variant> and ordered comparisons.
// Python 3 typically raises TypeError for < comparisons between incompatible types.
// We'll try to mimic some of that, but might need to be simpler for C++ std::set.
// A common approach for std::variant in std::map/set keys is to compare index() first, then value.
bool operator<(const eppx_variant& lhs, const eppx_variant& rhs) {
    if (lhs.index() != rhs.index()) {
        // Different types: Python 3 would raise TypeError for most cross-type comparisons.
        // For std::set stability, we need a consistent order. Ordering by index is one way.
        // However, to be more Python-like for direct user comparisons, we should only allow compatible types.
        // This simplified version relies on index for std::set and expects direct comparisons to be type-checked by user or calling code.
        return lhs.index() < rhs.index();
    }
    // Same types
    return std::visit([&rhs](const auto& lhs_val) -> bool {
        using T = std::decay_t<decltype(lhs_val)>;
        const T& rhs_val = std::get<T>(rhs); // Get same type from rhs

        if constexpr (std::is_same_v<T, std::nullptr_t>) return false; // None < None is false
        else if constexpr (std::is_same_v<T, bool>) return lhs_val < rhs_val; // false < true
        else if constexpr (std::is_same_v<T, long long>) return lhs_val < rhs_val;
        else if constexpr (std::is_same_v<T, double>) return lhs_val < rhs_val;
        else if constexpr (std::is_same_v<T, std::string>) return lhs_val < rhs_val;
        else if constexpr (std::is_same_v<T, std::complex<double>>) {
            // Lexicographical compare for complex
            if (lhs_val.real() != rhs_val.real()) {
                return lhs_val.real() < rhs_val.real();
            }
            return lhs_val.imag() < rhs_val.imag();
        }
        else if constexpr (std::is_same_v<T, eppx_bytearray>) return lhs_val.data < rhs_val.data; // Lexicographical
        else if constexpr (std::is_same_v<T, eppx_bytes>) return lhs_val.data < rhs_val.data;     // Lexicographical
        else if constexpr (std::is_same_v<T, eppx_list_variant_t>) return lhs_val < rhs_val; // std::vector default operator<
        else if constexpr (std::is_same_v<T, eppx_tuple>) return lhs_val < rhs_val; // Uses eppx_tuple::operator<
        else if constexpr (std::is_same_v<T, eppx_frozenset>) return lhs_val < rhs_val; // Uses eppx_frozenset::operator<
        // Dicts and Sets are not comparable with < in Python
        else if constexpr (std::is_same_v<T, eppx_dict_variant_t> || std::is_same_v<T, eppx_set_variant_t>) {
            // This will likely cause issues if used directly in std::set without more specific error handling
            // or if Python code tries `dict1 < dict2`. For now, make them non-comparable for '<'.
            // A more robust solution would throw or ensure these aren't used in ordered contexts without care.
            return false; // Or throw, but std::set needs this to compile.
        }
        // Fallback for any other unhandled types, though all variant types should be covered.
        return false;
    }, lhs);
}


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

void eppx_print_single(const std::complex<double>& c) {
    std::cout << "(" << c.real() << (c.imag() >= 0 ? "+" : "") << c.imag() << "j)";
}

// Forward declaration for printing eppx_variant itself (used in list/dict printing)
void eppx_print_single(const eppx_variant& v);


void eppx_print_single(const eppx_tuple& tpl) {
    std::cout << "(";
    for (size_t i = 0; i < tpl.elements.size(); ++i) {
        if (i > 0) std::cout << ", ";
        eppx_print_single(tpl.elements[i]);
    }
    if (tpl.elements.size() == 1) { // Python prints (1,) for single element tuple
        std::cout << ",";
    }
    std::cout << ")";
}

void eppx_print_single(const eppx_frozenset& fs) {
    std::cout << "frozenset({";
    bool first = true;
    for (const auto& item : fs.data) {
        if (!first) std::cout << ", ";
        eppx_print_single(item);
        first = false;
    }
    std::cout << "})";
}

void eppx_print_single(const eppx_set_variant_t& s) {
    if (s.empty()) {
        std::cout << "set()"; // Python prints set() for empty set
        return;
    }
    std::cout << "{";
    bool first = true;
    for (const auto& item : s) {
        if (!first) std::cout << ", ";
        eppx_print_single(item);
        first = false;
    }
    std::cout << "}";
}


void eppx_print_single(const eppx_list_variant_t& list_val) {
    std::cout << "[";
    for (size_t i = 0; i < list_val.size(); ++i) {
        if (i > 0) std::cout << ", ";
        eppx_print_single(list_val[i]); // Recursively call for variant
    }
    std::cout << "]";
}

void eppx_print_single(const eppx_dict_variant_t& dict_val) {
    std::cout << "{";
    bool first = true;
    for (const auto& pair : dict_val) {
        if (!first) std::cout << ", ";
        // Print string key (assuming keys are always strings for now)
        // If keys could be other eppx_variant types, this would need eppx_print_single(pair.first)
        std::cout << "'" << pair.first << "': ";
        eppx_print_single(pair.second); // Recursively call for variant value
        first = false;
    }
    std::cout << "}";
}

// Actual definition for printing eppx_variant, now that list/dict printers are declared
void eppx_print_single(const eppx_variant& v) {
    std::visit([](const auto& arg) {
        // Check if the type is one of those that has a specific eppx_print_single overload
        // This avoids infinite recursion if a type doesn't have its own overload.
        // For basic types like long long, double, std::string, bool, std::cout << arg works directly.
        // For custom types like eppx_bytearray, eppx_bytes, eppx_list_variant_t, eppx_dict_variant_t,
        // their specific overloads of eppx_print_single are called.
        // If std::nullptr_t, it's handled by the generic std::cout << arg (which might print 0 or not compile well without specific handling).
        // Let's ensure nullptr_t is handled nicely.
        using T = std::decay_t<decltype(arg)>;
        if constexpr (std::is_same_v<T, std::nullptr_t>) {
            std::cout << "None";
        } else if constexpr (std::is_same_v<T, eppx_list_variant_t> ||
                           std::is_same_v<T, eppx_dict_variant_t> ||
                           std::is_same_v<T, eppx_bytearray> ||
                           std::is_same_v<T, eppx_bytes> ||
                           std::is_same_v<T, std::complex<double>> ||
                           std::is_same_v<T, eppx_tuple> ||          // Added tuple
                           std::is_same_v<T, eppx_frozenset> ||    // Added frozenset
                           std::is_same_v<T, eppx_set_variant_t>    // Added set
                           ) {
            eppx_print_single(arg); // Call specific overload
        }
         else { // Default for primitive types like long long, double, string, bool
            std::cout << arg; // Default for primitive types
        }
    }, v);
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
        } else if constexpr (std::is_same_v<T, std::nullptr_t>) {
            return "<class 'NoneType'>";
        } else if constexpr (std::is_same_v<T, eppx_list_variant_t>) {
            return "<class 'list'>";
        } else if constexpr (std::is_same_v<T, eppx_dict_variant_t>) {
            return "<class 'dict'>";
        } else if constexpr (std::is_same_v<T, std::complex<double>>) {
            return "<class 'complex'>";
        } else if constexpr (std::is_same_v<T, eppx_tuple>) {
            return "<class 'tuple'>";
        } else if constexpr (std::is_same_v<T, eppx_frozenset>) {
            return "<class 'frozenset'>";
        } else if constexpr (std::is_same_v<T, eppx_set_variant_t>) {
            return "<class 'set'>";
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

// id() function
template<typename T>
uintptr_t eppx_id(const T& obj) {
    // For non-variant types, or when the actual object is passed.
    return reinterpret_cast<uintptr_t>(&obj);
}

// Overload for eppx_variant.
// This attempts to get the address of the object *inside* the variant.
// Note: The address of the variant itself (eppx_id(var)) would be different from
// the address of its contained object (std::visit(... eppx_id(*ptr_to_contained_object) ...)).
// Python's id() behavior is based on the object's identity.
uintptr_t eppx_id(const eppx_variant& var) {
    return std::visit([](const auto& v) -> uintptr_t {
        // We need a pointer to the actual object stored in the variant.
        // 'v' here is a const reference to the contained object.
        return reinterpret_cast<uintptr_t>(&v);
    }, var);
}


// hash() function
// Helper to hash vector of bytes, used by bytearray and bytes
size_t hash_vector_unsigned_char(const std::vector<unsigned char>& vec) {
    size_t seed = vec.size();
    for(unsigned char x : vec) {
        seed ^= std::hash<unsigned char>{}(x) + 0x9e3779b9 + (seed << 6) + (seed >> 2);
    }
    return seed;
}

struct eppx_hash_visitor {
    size_t operator()(long long val) const {
        return std::hash<long long>{}(val);
    }
    size_t operator()(double val) const {
        return std::hash<double>{}(val);
    }
    size_t operator()(const std::string& val) const {
        return std::hash<std::string>{}(val);
    }
    size_t operator()(bool val) const {
        return std::hash<bool>{}(val);
    }
    size_t operator()(const eppx_bytearray& val) const {
        return hash_vector_unsigned_char(val.data);
    }
    size_t operator()(const eppx_bytes& val) const {
        return hash_vector_unsigned_char(val.data);
    }
    size_t operator()(std::nullptr_t) const {
        // Python's hash(None) is consistent. std::hash<std::nullptr_t> provides this.
        return std::hash<std::nullptr_t>{}(nullptr);
    }
    size_t operator()(const eppx_list_variant_t& /*val*/) const {
        // Python lists are unhashable
        throw std::runtime_error("TypeError: unhashable type: 'list'");
    }
    size_t operator()(const eppx_dict_variant_t& /*val*/) const {
        // Python dicts are unhashable
        throw std::runtime_error("TypeError: unhashable type: 'dict'");
    }
    size_t operator()(const std::complex<double>& val) const {
        // Hash complex numbers by combining hashes of real and imaginary parts
        size_t h1 = std::hash<double>{}(val.real());
        size_t h2 = std::hash<double>{}(val.imag());
        // Simple XOR combination, or use boost::hash_combine
        return h1 ^ (h2 << 1);
    }
    size_t operator()(const eppx_tuple& val) const {
        size_t seed = val.elements.size();
        for(const auto& elem : val.elements) {
            // Combine hashes of elements. Needs eppx_hash(const eppx_variant&)
            seed ^= eppx_hash(elem) + 0x9e3779b9 + (seed << 6) + (seed >> 2);
        }
        return seed;
    }
    size_t operator()(const eppx_frozenset& val) const {
        size_t seed = 0;
        // Order-independent hash for frozenset (XOR sum is one simple way)
        // Elements in std::set are already sorted, so iterating gives consistent order.
        for(const auto& elem : val.data) {
            seed ^= eppx_hash(elem) + 0x9e3779b9 + (seed << 6) + (seed >> 2);
        }
        return seed;
    }
    size_t operator()(const eppx_set_variant_t& /*val*/) const {
        // Python sets are unhashable
        throw std::runtime_error("TypeError: unhashable type: 'set'");
    }
    // For other types like std::vector, std::map, std::set if they were part of eppx_variant:
    // Python lists, dicts, sets are unhashable. Tuples are hashable if elements are.
    // We would need to throw a runtime error here for those types.
    // For now, eppx_variant only holds types for which we've defined hashing.
    // If a type not listed here is added to eppx_variant, this visitor would need to be updated.
};

long long eppx_hash(const eppx_variant& var) {
    // Python's hash() can return negative numbers, so result should be long long.
    // std::hash returns size_t, which is unsigned. We'll cast.
    return static_cast<long long>(std::visit(eppx_hash_visitor{}, var));
}


// --- Static Attribute Access Machinery ---

struct ClassStaticMemberAccessors {
    // Using std::function to allow different signatures if needed, though for static members they are simpler.
    // For 'has', we could just check if the name exists in get/set maps.
    std::map<std::string, std::function<bool()>> has_attr_s_funcs; // Check if static member exists
    std::map<std::string, std::function<eppx_variant()>> get_attr_s_funcs;
    std::map<std::string, std::function<void(eppx_variant)>> set_attr_s_funcs;
    // del_attr_s_funcs are omitted as static members cannot be deleted.
};

// Global registry for static member accessors
// This map will be populated by generated code for each class.
// Ensure this is defined only once (e.g., in a .cpp file if headers are included multiple times,
// or use inline/static techniques if purely header-only for template/inline functions).
// For simplicity in a single header context for now:
inline std::map<std::string, ClassStaticMemberAccessors>& get_global_class_static_accessors() {
    static std::map<std::string, ClassStaticMemberAccessors> accessors_map;
    return accessors_map;
}


bool eppx_has_static_attr(const std::string& class_name, const std::string& member_name) {
    const auto& global_accessors = get_global_class_static_accessors();
    auto class_it = global_accessors.find(class_name);
    if (class_it != global_accessors.end()) {
        const auto& accessors = class_it->second;
        // Prefer checking dedicated has_attr_s_funcs if populated, otherwise infer from getters/setters.
        auto member_it_has = accessors.has_attr_s_funcs.find(member_name);
        if (member_it_has != accessors.has_attr_s_funcs.end()) {
            return (member_it_has->second)(); // Call the specific checker
        }
        // Fallback: if a getter or setter exists, attribute is considered to exist.
        // This is useful if specific has_attr functions are not generated for every member.
        if (accessors.get_attr_s_funcs.count(member_name) > 0 || accessors.set_attr_s_funcs.count(member_name) > 0) {
            return true;
        }
    }
    return false;
}

eppx_variant eppx_get_static_attr(const std::string& class_name, const std::string& member_name) {
    const auto& global_accessors = get_global_class_static_accessors();
    auto class_it = global_accessors.find(class_name);
    if (class_it != global_accessors.end()) {
        const auto& accessors = class_it->second;
        auto member_it = accessors.get_attr_s_funcs.find(member_name);
        if (member_it != accessors.get_attr_s_funcs.end()) {
            return (member_it->second)(); // Call the getter
        }
    }
    throw std::runtime_error("AttributeError: class '" + class_name + "' has no static attribute '" + member_name + "'");
}

eppx_variant eppx_get_static_attr_with_default(const std::string& class_name, const std::string& member_name, eppx_variant default_val) {
    if (eppx_has_static_attr(class_name, member_name)) { // Reuse our hasattr logic
        // Need to be careful about exceptions from eppx_get_static_attr if has_attr was true due to a setter only.
        // For now, assume if has_attr is true, a getter should ideally exist.
        const auto& global_accessors = get_global_class_static_accessors();
        auto class_it = global_accessors.find(class_name);
        if (class_it != global_accessors.end()) {
            const auto& accessors = class_it->second;
            auto member_it = accessors.get_attr_s_funcs.find(member_name);
            if (member_it != accessors.get_attr_s_funcs.end()) {
                 return (member_it->second)();
            }
        }
    }
    return default_val;
}

void eppx_set_static_attr(const std::string& class_name, const std::string& member_name, eppx_variant value) {
    auto& global_accessors = get_global_class_static_accessors(); // Non-const access
    auto class_it = global_accessors.find(class_name);
    if (class_it != global_accessors.end()) {
        // Note: class_it->second is ClassStaticMemberAccessors.
        // If it was marked const, we couldn't call non-const functions on its members (if any were non-const).
        // However, std::function objects themselves are copyable/movable, the functions they wrap define constness.
        auto member_it = class_it->second.set_attr_s_funcs.find(member_name);
        if (member_it != class_it->second.set_attr_s_funcs.end()) {
            (member_it->second)(value); // Call the setter
            return;
        }
    }
    throw std::runtime_error("AttributeError: class '" + class_name + "' has no static attribute '" + member_name + "' to set, or attribute is read-only.");
}

void eppx_del_static_attr(const std::string& class_name, const std::string& member_name) {
    // Static members cannot be deleted in C++ in this manner.
    throw std::runtime_error("AttributeError: cannot delete static attribute '" + member_name + "' from class '" + class_name + "'");
}

// --- End Static Attribute Access Machinery ---

// Helper function to convert eppx_variant to double for complex()
double variant_to_double(const eppx_variant& var, const char* arg_name) {
    return std::visit([arg_name](const auto& v) -> double {
        using T = std::decay_t<decltype(v)>;
        if constexpr (std::is_same_v<T, long long>) {
            return static_cast<double>(v);
        } else if constexpr (std::is_same_v<T, double>) {
            return v;
        } else if constexpr (std::is_same_v<T, bool>) {
            return v ? 1.0 : 0.0;
        } else {
            throw std::runtime_error(std::string("TypeError: ") + arg_name + " must be a number, not " + eppx_type(var));
        }
    }, var);
}

// complex() built-in function
eppx_variant eppx_complex() {
    return eppx_variant(std::complex<double>(0.0, 0.0));
}

eppx_variant eppx_complex(const eppx_variant& real_arg) {
    double real_val = variant_to_double(real_arg, "complex() real arg");
    return eppx_variant(std::complex<double>(real_val, 0.0));
}

eppx_variant eppx_complex(const eppx_variant& real_arg, const eppx_variant& imag_arg) {
    double real_val = variant_to_double(real_arg, "complex() real arg");
    double imag_val = variant_to_double(imag_arg, "complex() imag arg");
    return eppx_variant(std::complex<double>(real_val, imag_val));
}

// list() built-in function
eppx_variant eppx_list() {
    return eppx_variant(eppx_list_variant_t{});
}

eppx_variant eppx_list(const eppx_variant& iterable) {
    eppx_list_variant_t result_list;
    std::visit([&result_list](const auto& arg) {
        using T = std::decay_t<decltype(arg)>;
        if constexpr (std::is_same_v<T, std::string>) {
            for (char c : arg) {
                result_list.push_back(eppx_variant(std::string(1, c)));
            }
        } else if constexpr (std::is_same_v<T, eppx_bytes>) {
            for (unsigned char uc : arg.data) {
                result_list.push_back(eppx_variant(static_cast<long long>(uc)));
            }
        } else if constexpr (std::is_same_v<T, eppx_bytearray>) {
            for (unsigned char uc : arg.data) {
                result_list.push_back(eppx_variant(static_cast<long long>(uc)));
            }
        } else if constexpr (std::is_same_v<T, eppx_list_variant_t>) {
            result_list = arg; // Shallow copy as per Python's list(iterable)
        }
        // TODO: Add support for other iterable types (tuples, sets, dict keys) when they are added to eppx_variant
        else {
            throw std::runtime_error("TypeError: '" + eppx_type(eppx_variant(arg)) + "' object is not iterable");
        }
    }, iterable);
    return eppx_variant(result_list);
}


// dir() and vars() implementations (before type constructors for better readability)
// ... (dir and vars functions remain here) ...

// --- set(), frozenset(), tuple() constructors ---

// Helper to iterate eppx_variant iterable and fill a collection
template<typename OutputCollection, typename ElementProcessor>
void eppx_fill_from_iterable(const eppx_variant& iterable, OutputCollection& collection, ElementProcessor process_element) {
    std::visit([&](const auto& arg) {
        using T = std::decay_t<decltype(arg)>;
        if constexpr (std::is_same_v<T, std::string>) {
            for (char c : arg) { process_element(eppx_variant(std::string(1, c))); }
        } else if constexpr (std::is_same_v<T, eppx_bytes>) {
            for (unsigned char uc : arg.data) { process_element(eppx_variant(static_cast<long long>(uc))); }
        } else if constexpr (std::is_same_v<T, eppx_bytearray>) {
            for (unsigned char uc : arg.data) { process_element(eppx_variant(static_cast<long long>(uc))); }
        } else if constexpr (std::is_same_v<T, eppx_list_variant_t> || std::is_same_v<T, eppx_tuple> || std::is_same_v<T, eppx_set_variant_t> || std::is_same_v<T, eppx_frozenset>) {
            // If it's one of our collection types, iterate its elements
            const auto* elements_ptr = &arg; // Placeholder to access elements generically if possible
            if constexpr (std::is_same_v<T, eppx_list_variant_t>) {
                 for (const auto& item : arg) { process_element(item); }
            } else if constexpr (std::is_same_v<T, eppx_tuple>) {
                 for (const auto& item : arg.elements) { process_element(item); }
            } else if constexpr (std::is_same_v<T, eppx_set_variant_t> || std::is_same_v<T, eppx_frozenset>) {
                 for (const auto& item : arg.data) { process_element(item); }
            }
        }
        // TODO: Add dict (iterate keys)
        else {
            throw std::runtime_error("TypeError: '" + eppx_type(iterable) + "' object is not iterable for set/tuple/frozenset construction");
        }
    }, iterable);
}

// set()
eppx_variant eppx_set() {
    return eppx_variant(eppx_set_variant_t{});
}
eppx_variant eppx_set(const eppx_variant& iterable) {
    eppx_set_variant_t temp_set;
    eppx_fill_from_iterable(iterable, temp_set, [&](const eppx_variant& item) {
        eppx_hash(item); // Ensure item is hashable (will throw if not)
        temp_set.insert(item);
    });
    return eppx_variant(temp_set);
}

// frozenset()
eppx_variant eppx_frozenset_constructor() { // Renamed to avoid conflict if eppx_frozenset is a type
    return eppx_variant(eppx_frozenset{});
}
eppx_variant eppx_frozenset_constructor(const eppx_variant& iterable) {
    eppx_set_variant_t temp_set_data; // Use std::set to build frozenset data for uniqueness and order
    eppx_fill_from_iterable(iterable, temp_set_data, [&](const eppx_variant& item) {
        eppx_hash(item); // Ensure item is hashable
        temp_set_data.insert(item);
    });
    return eppx_variant(eppx_frozenset{std::move(temp_set_data)});
}

// tuple()
eppx_variant eppx_tuple_constructor() { // Renamed
    return eppx_variant(eppx_tuple{});
}
eppx_variant eppx_tuple_constructor(const eppx_variant& iterable) {
    eppx_list_variant_t temp_elements;
    eppx_fill_from_iterable(iterable, temp_elements, [&](const eppx_variant& item) {
        temp_elements.push_back(item); // For tuple, order is preserved, duplicates allowed
    });
    return eppx_variant(eppx_tuple{std::move(temp_elements)});
}

// --- End of set(), frozenset(), tuple() constructors ---

// --- dict() constructor functions ---
eppx_variant eppx_dict() {
    return eppx_variant(eppx_dict_variant_t{});
}

// Helper for dict(iterable_of_pairs)
eppx_variant eppx_dict_from_iterable_of_pairs(const eppx_list_variant_t& items_list) {
    eppx_dict_variant_t temp_dict;
    for (const auto& item_variant : items_list) {
        std::visit([&temp_dict](const auto& pair_like) {
            using PairType = std::decay_t<decltype(pair_like)>;
            if constexpr (std::is_same_v<PairType, eppx_list_variant_t> || std::is_same_v<PairType, eppx_tuple>) {
                const auto& elements = (std::is_same_v<PairType, eppx_list_variant_t>) ? pair_like : pair_like.elements;
                if (elements.size() == 2) {
                    // Key must be string for std::map<std::string, ...>
                    // In a more advanced system, keys could be any hashable eppx_variant.
                    std::string key_str = std::visit([](const auto& key_val) -> std::string {
                        using KeyValueT = std::decay_t<decltype(key_val)>;
                        if constexpr (std::is_same_v<KeyValueT, std::string>) return key_val;
                        // Add conversions from other hashable types to string if desired, or throw.
                        // For now, only string keys are directly supported for map.
                        // Python dict keys must be hashable.
                        // eppx_hash(eppx_variant(key_val)); // Check hashability
                        throw std::runtime_error("TypeError: dict keys must be strings (for now)");
                        // return eppx_to_string_key_representation(key_val); // Placeholder
                    }, elements[0]);
                    temp_dict[key_str] = elements[1];
                } else {
                    throw std::runtime_error("ValueError: dictionary update sequence element has bad length (expected 2)");
                }
            } else {
                throw std::runtime_error("TypeError: cannot convert dictionary update sequence element to a sequence");
            }
        }, item_variant);
    }
    return eppx_variant(temp_dict);
}

// For dict(mapping) or dict(**kwargs) - kwargs are handled by codegen building a map literal.
// This function handles dict(arg) where arg can be a dict or list of pairs.
eppx_variant eppx_dict_constructor(const eppx_variant& arg) { // Renamed from eppx_dict
    return std::visit([](const auto& val) -> eppx_variant {
        using T = std::decay_t<decltype(val)>;
        if constexpr (std::is_same_v<T, eppx_dict_variant_t>) {
            return eppx_variant(val); // Return a copy
        } else if constexpr (std::is_same_v<T, eppx_list_variant_t>) {
            return eppx_dict_from_iterable_of_pairs(val);
        }
        // TODO: Could add other types that behave like mappings or iterables of pairs
        else {
            throw std::runtime_error("TypeError: cannot convert dictionary update sequence element to a sequence or mapping");
        }
    }, arg);
}

// --- End of dict() constructor functions ---


// dir() and vars() implementations
eppx_variant eppx_dir_static(const std::string& class_name) {
    eppx_list_variant_t names_vec;
    const auto& global_accessors = get_global_class_static_accessors();
    auto class_it = global_accessors.find(class_name);
    if (class_it != global_accessors.end()) {
        const auto& accessors = class_it->second;
        for (const auto& pair : accessors.has_attr_s_funcs) {
            names_vec.push_back(eppx_variant(pair.first));
        }
        // Python's dir also includes default method names, inherited names etc.
        // For now, just static members explicitly registered.
        // Sort for consistent output, like Python's dir()
        std::sort(names_vec.begin(), names_vec.end(), [](const eppx_variant& a, const eppx_variant& b){
            return std::get<std::string>(a) < std::get<std::string>(b);
        });
    } else {
        throw std::runtime_error("NameError: class '" + class_name + "' not found for dir()");
    }
    return names_vec;
}

eppx_variant eppx_dir_global() {
    // For now, dir() without arguments is not listing current scope variables.
    // Python's dir() on its own lists names in the current local scope.
    return eppx_list_variant_t{}; // Return empty list
}

eppx_variant eppx_vars_static(const std::string& class_name) {
    eppx_dict_variant_t vars_map;
    const auto& global_accessors = get_global_class_static_accessors();
    auto class_it = global_accessors.find(class_name);
    if (class_it != global_accessors.end()) {
        const auto& accessors = class_it->second;
        for (const auto& pair : accessors.get_attr_s_funcs) {
            // Only include attributes that have getters (typically static data members).
            // Static methods might not have simple getters in get_attr_s_funcs.
            vars_map[pair.first] = pair.second(); // Call getter to get current value
        }
    } else {
        throw std::runtime_error("NameError: class '" + class_name + "' not found for vars()");
    }
    return vars_map;
}

eppx_variant eppx_vars_global() {
    // Python's vars() on its own is like locals(). Not implemented for current scope yet.
    return eppx_dict_variant_t{}; // Return empty dict
}


// Object attribute functions (simplified stubs for instance attributes - to be implemented later)
bool eppx_hasattr(const eppx_variant& obj, const std::string& name); // Declaration only
eppx_variant eppx_getattr(const eppx_variant& obj, const std::string& name); // Declaration only
eppx_variant eppx_getattr(const eppx_variant& obj, const std::string& name, const eppx_variant& default_value); // Declaration only
void eppx_setattr(const eppx_variant& obj, const std::string& name, const eppx_variant& value); // Declaration only
void eppx_delattr(const eppx_variant& obj, const std::string& name); // Declaration only


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
