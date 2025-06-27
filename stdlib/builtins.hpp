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
#include <fstream>
#include <memory>
#include <stdexcept>
#include <fstream>
#include <optional>
#include <tuple>
#include <cmath>
#include <cstdint>

// Forward declaration for recursive variant
struct eppx_variant;

// Basic variant type for E++ values - recursive to support nested lists
using eppx_variant_base = std::variant<long long, std::string, double, bool, std::vector<eppx_variant>>;

struct eppx_variant : public eppx_variant_base {
    using eppx_variant_base::eppx_variant_base;
    using eppx_variant_base::operator=;
};

// Helper functions for variant conversion
inline double variant_to_double(const eppx_variant& v) {
    if (std::holds_alternative<double>(v)) {
        return std::get<double>(v);
    } else if (std::holds_alternative<long long>(v)) {
        return static_cast<double>(std::get<long long>(v));
    } else if (std::holds_alternative<bool>(v)) {
        return std::get<bool>(v) ? 1.0 : 0.0;
    }
    return 0.0;
}

inline long long variant_to_ll(const eppx_variant& v) {
    if (std::holds_alternative<long long>(v)) {
        return std::get<long long>(v);
    } else if (std::holds_alternative<double>(v)) {
        return static_cast<long long>(std::get<double>(v));
    } else if (std::holds_alternative<bool>(v)) {
        return std::get<bool>(v) ? 1LL : 0LL;
    }
    return 0LL;
}

inline std::string variant_to_string(const eppx_variant& v) {
    if (std::holds_alternative<std::string>(v)) {
        return std::get<std::string>(v);
    } else if (std::holds_alternative<long long>(v)) {
        return std::to_string(std::get<long long>(v));
    } else if (std::holds_alternative<double>(v)) {
        return std::to_string(std::get<double>(v));
    } else if (std::holds_alternative<bool>(v)) {
        return std::get<bool>(v) ? "True" : "False";
    }
    return "";
}

inline bool variant_to_bool(const eppx_variant& v) {
    if (std::holds_alternative<bool>(v)) {
        return std::get<bool>(v);
    } else if (std::holds_alternative<long long>(v)) {
        return std::get<long long>(v) != 0;
    } else if (std::holds_alternative<double>(v)) {
        return std::get<double>(v) != 0.0;
    } else if (std::holds_alternative<std::string>(v)) {
        return !std::get<std::string>(v).empty();
    } else if (std::holds_alternative<std::vector<eppx_variant>>(v)) {
        return !std::get<std::vector<eppx_variant>>(v).empty();
    }
    return false;
}

// Output operator for eppx_variant
std::ostream& operator<<(std::ostream& os, const eppx_variant& var) {
    std::visit([&os](const auto& value) {
        os << value;
    }, var);
    return os;
}

// Output operator for std::pair (for divmod results)
template<typename T1, typename T2>
std::ostream& operator<<(std::ostream& os, const std::pair<T1, T2>& p) {
    os << "(" << p.first << ", " << p.second << ")";
    return os;
}

// Forward declarations for output operators (defined after classes)

// Arithmetic operators for eppx_variant
eppx_variant operator*(const eppx_variant& left, const eppx_variant& right) {
    if (std::holds_alternative<long long>(left) && std::holds_alternative<long long>(right)) {
        return std::get<long long>(left) * std::get<long long>(right);
    } else if (std::holds_alternative<double>(left) || std::holds_alternative<double>(right)) {
        return variant_to_double(left) * variant_to_double(right);
    }
    return 0LL; // fallback
}

eppx_variant operator%(const eppx_variant& left, const eppx_variant& right) {
    long long l = variant_to_ll(left);
    long long r = variant_to_ll(right);
    return l % r;
}

eppx_variant operator+(const eppx_variant& left, const eppx_variant& right) {
    if (std::holds_alternative<long long>(left) && std::holds_alternative<long long>(right)) {
        return std::get<long long>(left) + std::get<long long>(right);
    } else if (std::holds_alternative<double>(left) || std::holds_alternative<double>(right)) {
        return variant_to_double(left) + variant_to_double(right);
    } else if (std::holds_alternative<std::string>(left) || std::holds_alternative<std::string>(right)) {
        return variant_to_string(left) + variant_to_string(right);
    }
    return 0LL; // fallback
}

eppx_variant operator-(const eppx_variant& left, const eppx_variant& right) {
    if (std::holds_alternative<long long>(left) && std::holds_alternative<long long>(right)) {
        return std::get<long long>(left) - std::get<long long>(right);
    } else if (std::holds_alternative<double>(left) || std::holds_alternative<double>(right)) {
        return variant_to_double(left) - variant_to_double(right);
    }
    return 0LL; // fallback
}

eppx_variant operator/(const eppx_variant& left, const eppx_variant& right) {
    double l = variant_to_double(left);
    double r = variant_to_double(right);
    return l / r;
}

// Comparison operators for eppx_variant
bool operator==(const eppx_variant& left, const eppx_variant& right) {
    if (left.index() != right.index()) {
        return false;
    }
    return std::visit([](const auto& l, const auto& r) {
        return l == r;
    }, left, right);
}

bool operator!=(const eppx_variant& left, const eppx_variant& right) {
    return !(left == right);
}

bool operator<(const eppx_variant& left, const eppx_variant& right) {
    if (std::holds_alternative<long long>(left) && std::holds_alternative<long long>(right)) {
        return std::get<long long>(left) < std::get<long long>(right);
    } else if (std::holds_alternative<double>(left) || std::holds_alternative<double>(right)) {
        return variant_to_double(left) < variant_to_double(right);
    } else if (std::holds_alternative<std::string>(left) && std::holds_alternative<std::string>(right)) {
        return std::get<std::string>(left) < std::get<std::string>(right);
    }
    return false;
}

bool operator<=(const eppx_variant& left, const eppx_variant& right) {
    return left < right || left == right;
}

bool operator>(const eppx_variant& left, const eppx_variant& right) {
    return !(left <= right);
}

bool operator>=(const eppx_variant& left, const eppx_variant& right) {
    return !(left < right);
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

// Range function overloads
std::vector<long long> eppx_range(long long start, long long stop) {
    std::vector<long long> result;
    if (start < stop) {
        result.reserve(stop - start);
        for (long long i = start; i < stop; ++i) {
            result.push_back(i);
        }
    }
    return result;
}

std::vector<long long> eppx_range(long long start, long long stop, long long step) {
    std::vector<long long> result;
    if (step > 0 && start < stop) {
        for (long long i = start; i < stop; i += step) {
            result.push_back(i);
        }
    } else if (step < 0 && start > stop) {
        for (long long i = start; i > stop; i += step) {
            result.push_back(i);
        }
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
    return std::string("0b") + (n < 0 ? "-" : "") + binary;
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

// Specialized sum for eppx_variant vectors
template<>
eppx_variant eppx_sum(const std::vector<eppx_variant>& container) {
    eppx_variant sum = 0LL;
    for (const auto& item : container) {
        if (std::holds_alternative<long long>(sum) && std::holds_alternative<long long>(item)) {
            sum = std::get<long long>(sum) + std::get<long long>(item);
        } else if (std::holds_alternative<double>(sum) || std::holds_alternative<double>(item)) {
            double s = variant_to_double(sum);
            double i = variant_to_double(item);
            sum = s + i;
        }
    }
    return sum;
}

template<typename Container>
bool eppx_all(const Container& container) {
    return std::all_of(container.begin(), container.end(), 
                      [](const auto& item) { return variant_to_bool(item); });
}

template<typename Container>
bool eppx_any(const Container& container) {
    return std::any_of(container.begin(), container.end(), 
                      [](const auto& item) { return variant_to_bool(item); });
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

// List conversion function (alias for eppx_to_list)
template<typename Container>
auto eppx_list(const Container& container) -> std::vector<typename Container::value_type> {
    return std::vector<typename Container::value_type>(container.begin(), container.end());
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

// Note: isinstance, callable, and hasattr are implemented later with improved versions

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

// Higher-order functions (simplified implementations) - moved to later section

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

// Length function
template<typename Container>
auto eppx_len(const Container& container) -> size_t {
    return container.size();
}

// String length specialization
size_t eppx_len(const std::string& str) {
    return str.length();
}

// eppx_variant length specialization
size_t eppx_len(const eppx_variant& var) {
    if (std::holds_alternative<std::string>(var)) {
        return std::get<std::string>(var).length();
    } else if (std::holds_alternative<std::vector<eppx_variant>>(var)) {
        return std::get<std::vector<eppx_variant>>(var).size();
    }
    // For other types, return 0 or throw an error
    throw std::runtime_error("len() not supported for this type");
}

// Multi-argument min/max functions
template<typename T, typename... Args>
auto eppx_min(T first, Args... args) -> T {
    return std::min({first, static_cast<T>(args)...});
}

template<typename T, typename... Args>
auto eppx_max(T first, Args... args) -> T {
    return std::max({first, static_cast<T>(args)...});
}

// Container min/max functions
template<typename Container>
auto eppx_max(const Container& container) -> typename Container::value_type {
    return *std::max_element(container.begin(), container.end());
}

template<typename Container>
auto eppx_min(const Container& container) -> typename Container::value_type {
    return *std::min_element(container.begin(), container.end());
}

// File I/O functions
class EppxFile {
private:
    std::string filepath;
    std::string mode;
    std::fstream file_stream;
    bool is_open;
    std::streampos position;

public:
    EppxFile(const std::string& path, const std::string& file_mode) 
        : filepath(path), mode(file_mode), is_open(false), position(0) {}

    bool open() {
        std::ios_base::openmode open_mode = std::ios_base::in;
        
        if (mode.find('w') != std::string::npos) {
            open_mode = std::ios_base::out | std::ios_base::trunc;
        } else if (mode.find('a') != std::string::npos) {
            open_mode = std::ios_base::out | std::ios_base::app;
        } else if (mode.find('r') != std::string::npos) {
            open_mode = std::ios_base::in;
        }
        
        if (mode.find('+') != std::string::npos) {
            open_mode |= std::ios_base::in | std::ios_base::out;
        }
        
        if (mode.find('b') != std::string::npos) {
            open_mode |= std::ios_base::binary;
        }

        file_stream.open(filepath, open_mode);
        is_open = file_stream.is_open();
        return is_open;
    }

    std::string read(int size = -1) {
        if (!is_open) {
            throw std::runtime_error("I/O operation on closed file");
        }
        
        std::string content;
        if (size == -1) {
            // Read entire file
            file_stream.seekg(0, std::ios::end);
            content.reserve(file_stream.tellg());
            file_stream.seekg(0, std::ios::beg);
            content.assign((std::istreambuf_iterator<char>(file_stream)),
                          std::istreambuf_iterator<char>());
        } else {
            // Read specific number of characters
            content.resize(size);
            file_stream.read(&content[0], size);
            content.resize(file_stream.gcount());
        }
        return content;
    }

    std::string readline(int size = -1) {
        if (!is_open) {
            throw std::runtime_error("I/O operation on closed file");
        }
        
        std::string line;
        std::getline(file_stream, line);
        if (size != -1 && line.length() > size) {
            line = line.substr(0, size);
        }
        return line + "\n"; // Python readline includes newline
    }

    std::vector<std::string> readlines(int hint = -1) {
        if (!is_open) {
            throw std::runtime_error("I/O operation on closed file");
        }
        
        std::vector<std::string> lines;
        std::string line;
        int total_size = 0;
        
        while (std::getline(file_stream, line)) {
            line += "\n";
            if (hint != -1 && total_size + line.length() > hint) {
                break;
            }
            lines.push_back(line);
            total_size += line.length();
        }
        return lines;
    }

    int write(const std::string& data) {
        if (!is_open) {
            throw std::runtime_error("I/O operation on closed file");
        }
        
        file_stream << data;
        return data.length();
    }

    void writelines(const std::vector<std::string>& lines) {
        for (const auto& line : lines) {
            write(line);
        }
    }

    void close() {
        if (is_open) {
            file_stream.close();
            is_open = false;
        }
    }

    void flush() {
        if (is_open) {
            file_stream.flush();
        }
    }

    std::streampos seek(std::streamoff offset, std::ios_base::seekdir whence = std::ios_base::beg) {
        if (!is_open) {
            throw std::runtime_error("I/O operation on closed file");
        }
        
        file_stream.seekg(offset, whence);
        file_stream.seekp(offset, whence);
        return file_stream.tellg();
    }

    std::streampos tell() {
        if (!is_open) {
            throw std::runtime_error("I/O operation on closed file");
        }
        return file_stream.tellg();
    }

    bool readable() const {
        return is_open && (mode.find('r') != std::string::npos || mode.find('+') != std::string::npos);
    }

    bool writable() const {
        return is_open && (mode.find('w') != std::string::npos || 
                          mode.find('a') != std::string::npos || 
                          mode.find('+') != std::string::npos);
    }

    bool seekable() const {
        return is_open;
    }

    bool closed() const {
        return !is_open;
    }

    std::string get_mode() const {
        return mode;
    }

    std::string get_name() const {
        return filepath;
    }
};

// File I/O builtin functions
std::shared_ptr<EppxFile> eppx_open(const std::string& filepath, 
                                    const std::string& mode = "r",
                                    int buffering = -1,
                                    const std::string& encoding = "",
                                    const std::string& errors = "strict",
                                    const std::string& newline = "",
                                    bool closefd = true) {
    auto file_obj = std::make_shared<EppxFile>(filepath, mode);
    if (!file_obj->open()) {
        if (mode.find('r') != std::string::npos) {
            throw std::runtime_error("No such file or directory: '" + filepath + "'");
        } else {
            throw std::runtime_error("Could not open file: '" + filepath + "'");
        }
    }
    return file_obj;
}

// Context manager for files
// (Needed for Python-style with open(...) as ...)
template<typename FileType>
class EppxFileContextManager {
private:
    FileType file_obj;
    bool should_close;
public:
    EppxFileContextManager(FileType f) : file_obj(f), should_close(true) {}
    FileType& __enter__() { return file_obj; }
    bool __exit__(const std::string& exc_type = "", const std::string& exc_val = "", const std::string& exc_tb = "") {
        if (should_close && file_obj) {
            file_obj->close();
        }
        return false; // Don't suppress exceptions
    }
};

template<typename FileType>
EppxFileContextManager<FileType> eppx_with_file(FileType file_obj) {
    return EppxFileContextManager<FileType>(file_obj);
}

// String utility functions
std::string eppx_upper(const std::string& s) {
    std::string result = s;
    std::transform(result.begin(), result.end(), result.begin(), ::toupper);
    return result;
}

std::string eppx_upper(const eppx_variant& v) {
    return eppx_upper(variant_to_string(v));
}

std::string eppx_lower(const std::string& s) {
    std::string result = s;
    std::transform(result.begin(), result.end(), result.begin(), ::tolower);
    return result;
}

std::string eppx_lower(const eppx_variant& v) {
    return eppx_lower(variant_to_string(v));
}

// Iterator and generator support
template<typename T>
class EppxIterator {
private:
    std::vector<T> data;
    size_t current_index;

public:
    EppxIterator(const std::vector<T>& vec) : data(vec), current_index(0) {}
    
    bool has_next() const {
        return current_index < data.size();
    }
    
    T next() {
        if (!has_next()) {
            throw std::runtime_error("StopIteration");
        }
        return data[current_index++];
    }
    
    void reset() {
        current_index = 0;
    }
};

// Iterator factory function
template<typename T>
EppxIterator<T> eppx_iter(const std::vector<T>& iterable) {
    return EppxIterator<T>(iterable);
}

// next() builtin function
template<typename T>
T eppx_next(EppxIterator<T>& iterator) {
    return iterator.next();
}

// Generator base class
class EppxGenerator {
public:
    virtual ~EppxGenerator() = default;
    virtual eppx_variant next() = 0;
    virtual bool has_next() const = 0;
    virtual void reset() {}
};

// Simple range generator
class EppxRangeGenerator : public EppxGenerator {
private:
    long long current;
    long long stop;
    long long step;

public:
    EppxRangeGenerator(long long start, long long stop_val, long long step_val = 1)
        : current(start), stop(stop_val), step(step_val) {}

    eppx_variant next() override {
        if (!has_next()) {
            throw std::runtime_error("StopIteration");
        }
        long long value = current;
        current += step;
        return value;
    }

    bool has_next() const override {
        if (step > 0) {
            return current < stop;
        } else {
            return current > stop;
        }
    }

    void reset() override {
        // Would need to store original start value to implement reset
    }
};

// Generator expression support
template<typename Func, typename Iterable>
class EppxGeneratorExpression : public EppxGenerator {
private:
    Func transform_func;
    EppxIterator<typename Iterable::value_type> iterator;

public:
    EppxGeneratorExpression(Func func, const Iterable& iterable)
        : transform_func(func), iterator(iterable) {}

    eppx_variant next() override {
        if (!has_next()) {
            throw std::runtime_error("StopIteration");
        }
        auto value = iterator.next();
        return transform_func(value);
    }

    bool has_next() const override {
        return iterator.has_next();
    }
};

// Iterator support classes and functions
template<typename T>
class ListIterator {
private:
    const std::vector<T>& container;
    size_t index = 0;
    
public:
    ListIterator(const std::vector<T>& container) : container(container) {}
    
    bool has_next() const {
        return index < container.size();
    }
    
    T next() {
        if (index >= container.size()) {
            throw std::runtime_error("StopIteration");
        }
        return container[index++];
    }
};

// Iterator for eppx_variant vectors (most common case)
class IteratorLL {
private:
    std::vector<eppx_variant> data;
    size_t index = 0;

public:
    IteratorLL() = default;
    IteratorLL(const std::vector<eppx_variant>& container) : data(container) {}
    
    bool has_next() const {
        return index < data.size();
    }
    
    eppx_variant next() {
        if (index >= data.size()) {
            throw std::runtime_error("StopIteration");
        }
        return data[index++];
    }
};

// Iterator for string vectors
class IteratorStr {
private:
    std::vector<eppx_variant> data;
    size_t index = 0;

public:
    IteratorStr() = default;
    IteratorStr(const std::vector<eppx_variant>& container) : data(container) {}
    
    bool has_next() const {
        return index < data.size();
    }
    
    eppx_variant next() {
        if (index >= data.size()) {
            throw std::runtime_error("StopIteration");
        }
        return data[index++];
    }
};

// Global iterator storage (simplified approach)
static std::map<std::string, IteratorLL> ll_iterators;
static int iterator_counter = 0;

// iter() function for eppx_variant vectors
std::string iter(const std::vector<eppx_variant>& container) {
    std::string iter_id = "iter_" + std::to_string(iterator_counter++);
    ll_iterators[iter_id] = IteratorLL(container);
    return iter_id;
}

// next() function for generator objects
template<typename Generator>
auto next(Generator& gen) -> decltype(gen.next_value()) {
    return gen.next_value();
}

// next() function for eppx_variant iterators
eppx_variant next(const std::string& iter_id) {
    auto it = ll_iterators.find(iter_id);
    if (it != ll_iterators.end()) {
        return it->second.next();
    }
    throw std::runtime_error("Invalid iterator");
}

// ASCII representation function
std::string eppx_ascii(const eppx_variant& obj) {
    std::string result = "\"";
    std::string str_val = variant_to_string(obj);
    for (char c : str_val) {
        if (c >= 32 && c <= 126) {
            if (c == '"' || c == '\\') {
                result += "\\";
            }
            result += c;
        } else {
            result += "\\x" + eppx_hex(static_cast<unsigned char>(c)).substr(2);
        }
    }
    result += "\"";
    return result;
}

// Breakpoint function (debugging stub)
void eppx_breakpoint() {
    std::cout << "Breakpoint reached. Press Enter to continue..." << std::endl;
    std::cin.get();
}

// Bytearray class (simplified implementation)
class EppxByteArray {
private:
    std::vector<unsigned char> data;

public:
    EppxByteArray() = default;
    EppxByteArray(const std::string& str) {
        for (char c : str) {
            data.push_back(static_cast<unsigned char>(c));
        }
    }
    EppxByteArray(const std::vector<int>& values) {
        for (int val : values) {
            data.push_back(static_cast<unsigned char>(val));
        }
    }
    
    size_t size() const { return data.size(); }
    unsigned char& operator[](size_t index) { return data[index]; }
    const unsigned char& operator[](size_t index) const { return data[index]; }
    
    std::string to_string() const {
        std::string result;
        for (unsigned char byte : data) {
            result += static_cast<char>(byte);
        }
        return result;
    }
    
    void append(unsigned char byte) { data.push_back(byte); }
    void extend(const EppxByteArray& other) {
        data.insert(data.end(), other.data.begin(), other.data.end());
    }
};

// Bytes class (immutable byte sequence)
class EppxBytes {
private:
    std::vector<unsigned char> data;

public:
    EppxBytes() = default;
    EppxBytes(const std::string& str) {
        for (char c : str) {
            data.push_back(static_cast<unsigned char>(c));
        }
    }
    EppxBytes(const std::vector<int>& values) {
        for (int val : values) {
            data.push_back(static_cast<unsigned char>(val));
        }
    }
    
    size_t size() const { return data.size(); }
    const unsigned char& operator[](size_t index) const { return data[index]; }
    
    std::string to_string() const {
        std::string result;
        for (unsigned char byte : data) {
            result += static_cast<char>(byte);
        }
        return result;
    }
};

// Bytearray and bytes factory functions
EppxByteArray eppx_bytearray() {
    return EppxByteArray();
}

EppxByteArray eppx_bytearray(const std::string& str) {
    return EppxByteArray(str);
}

EppxByteArray eppx_bytearray(const std::vector<int>& values) {
    return EppxByteArray(values);
}

EppxBytes eppx_bytes() {
    return EppxBytes();
}

EppxBytes eppx_bytes(const std::string& str) {
    return EppxBytes(str);
}

EppxBytes eppx_bytes(const std::vector<int>& values) {
    return EppxBytes(values);
}

// Improved callable function
bool eppx_callable(const eppx_variant& obj) {
    // In a real implementation, this would check if the object has a __call__ method
    // For now, we'll return false for all basic types
    return false;
}

// Directory listing function
std::vector<std::string> eppx_dir(const eppx_variant& obj) {
    // Simplified implementation - would need full object introspection
    std::vector<std::string> attributes;
    
    if (std::holds_alternative<std::string>(obj)) {
        // String methods
        attributes = {"capitalize", "casefold", "center", "count", "encode", 
                     "endswith", "expandtabs", "find", "format", "index", 
                     "isalnum", "isalpha", "isascii", "isdecimal", "isdigit",
                     "isidentifier", "islower", "isnumeric", "isprintable",
                     "isspace", "istitle", "isupper", "join", "ljust", "lower",
                     "lstrip", "partition", "replace", "rfind", "rindex",
                     "rjust", "rpartition", "rsplit", "rstrip", "split",
                     "splitlines", "startswith", "strip", "swapcase", "title",
                     "translate", "upper", "zfill"};
    } else if (std::holds_alternative<std::vector<eppx_variant>>(obj)) {
        // List methods
        attributes = {"append", "clear", "copy", "count", "extend", "index",
                     "insert", "pop", "remove", "reverse", "sort"};
    } else if (std::holds_alternative<long long>(obj)) {
        // Integer methods
        attributes = {"bit_length", "conjugate", "denominator", "from_bytes",
                     "imag", "numerator", "real", "to_bytes"};
    } else if (std::holds_alternative<double>(obj)) {
        // Float methods
        attributes = {"as_integer_ratio", "conjugate", "fromhex", "hex",
                     "imag", "is_finite", "is_infinite", "is_integer", "real"};
    }
    
    return attributes;
}

// Divmod function
std::pair<long long, long long> eppx_divmod(long long a, long long b) {
    return std::make_pair(a / b, a % b);
}

std::pair<double, double> eppx_divmod(double a, double b) {
    double quotient = std::floor(a / b);
    double remainder = a - quotient * b;
    return std::make_pair(quotient, remainder);
}

// Improved enumerate function
template<typename Container>
std::vector<std::pair<size_t, typename Container::value_type>> eppx_enumerate(const Container& container, size_t start = 0) {
    std::vector<std::pair<size_t, typename Container::value_type>> result;
    size_t i = start;
    for (const auto& item : container) {
        result.emplace_back(i++, item);
    }
    return result;
}

// Eval function (simplified - would need full parser)
eppx_variant eppx_eval(const std::string& expression) {
    // Extremely simplified eval - only handles basic arithmetic
    // In a real implementation, this would parse and evaluate the expression
    try {
        // Try to parse as integer
        return static_cast<long long>(std::stoll(expression));
    } catch (...) {
        try {
            // Try to parse as float
            return std::stod(expression);
        } catch (...) {
            // Return as string if can't parse as number
            return expression;
        }
    }
}

// Exec function (stub)
void eppx_exec(const std::string& code) {
    // Stub implementation - would need full interpreter
    throw std::runtime_error("exec() not fully implemented");
}

// Format function
std::string eppx_format(const eppx_variant& value, const std::string& format_spec = "") {
    if (format_spec.empty()) {
        return variant_to_string(value);
    }
    
    // Simplified format implementation
    if (std::holds_alternative<double>(value)) {
        double val = std::get<double>(value);
        if (format_spec.find('.') != std::string::npos) {
            // Handle precision specifier like ".2f"
            size_t dot_pos = format_spec.find('.');
            if (dot_pos + 1 < format_spec.length()) {
                int precision = std::stoi(format_spec.substr(dot_pos + 1, 1));
                std::stringstream ss;
                ss << std::fixed << std::setprecision(precision) << val;
                return ss.str();
            }
        }
    }
    
    return variant_to_string(value);
}

// Globals function (stub)
std::map<std::string, eppx_variant> eppx_globals() {
    // Stub implementation - would need access to global scope
    std::map<std::string, eppx_variant> globals;
    globals["__name__"] = std::string("__main__");
    globals["__doc__"] = std::string("");
    return globals;
}

// Improved hasattr function
bool eppx_hasattr(const eppx_variant& obj, const std::string& name) {
    // Simplified implementation based on type
    if (std::holds_alternative<std::string>(obj)) {
        std::vector<std::string> string_attrs = {"upper", "lower", "strip", "split", "replace", "find"};
        return std::find(string_attrs.begin(), string_attrs.end(), name) != string_attrs.end();
    } else if (std::holds_alternative<std::vector<eppx_variant>>(obj)) {
        std::vector<std::string> list_attrs = {"append", "extend", "pop", "remove", "index", "count"};
        return std::find(list_attrs.begin(), list_attrs.end(), name) != list_attrs.end();
    }
    return false;
}

// Hash function
size_t eppx_hash(const eppx_variant& obj) {
    return std::visit([](const auto& value) -> size_t {
        using T = std::decay_t<decltype(value)>;
        if constexpr (std::is_same_v<T, long long>) {
            return std::hash<long long>{}(value);
        } else if constexpr (std::is_same_v<T, double>) {
            return std::hash<double>{}(value);
        } else if constexpr (std::is_same_v<T, std::string>) {
            return std::hash<std::string>{}(value);
        } else if constexpr (std::is_same_v<T, bool>) {
            return std::hash<bool>{}(value);
        } else {
            return 0; // Default hash for complex types
        }
    }, obj);
}

// Help function
void eppx_help(const eppx_variant& obj = eppx_variant{}) {
    if (std::holds_alternative<std::string>(obj)) {
        std::cout << "Help on built-in function " << std::get<std::string>(obj) << std::endl;
    } else {
        std::cout << "Welcome to E++ help system!" << std::endl;
        std::cout << "Type help(object) for help on a specific object." << std::endl;
    }
}

// ID function (memory address)
uintptr_t eppx_id(const eppx_variant& obj) {
    return reinterpret_cast<uintptr_t>(&obj);
}

// Improved isinstance function
bool eppx_isinstance(const eppx_variant& obj, const std::string& type_name) {
    std::string obj_type = eppx_type(obj);
    return obj_type.find(type_name) != std::string::npos;
}

// Issubclass function (stub)
bool eppx_issubclass(const std::string& subclass, const std::string& baseclass) {
    // Simplified implementation - would need full class hierarchy
    return subclass == baseclass;
}

// Locals function (stub)
std::map<std::string, eppx_variant> eppx_locals() {
    // Stub implementation - would need access to local scope
    return std::map<std::string, eppx_variant>();
}

// Memoryview class (simplified)
class EppxMemoryView {
private:
    const void* data_ptr;
    size_t data_size;
    std::string format;

public:
    EppxMemoryView(const void* ptr, size_t size, const std::string& fmt = "B") 
        : data_ptr(ptr), data_size(size), format(fmt) {}
    
    size_t size() const { return data_size; }
    const void* data() const { return data_ptr; }
    std::string get_format() const { return format; }
};

EppxMemoryView eppx_memoryview(const EppxBytes& bytes_obj) {
    return EppxMemoryView(bytes_obj.to_string().data(), bytes_obj.size());
}

// Object base class
class EppxObject {
public:
    virtual ~EppxObject() = default;
    virtual std::string to_string() const { return "<object>"; }
    virtual size_t hash() const { return reinterpret_cast<size_t>(this); }
};

EppxObject eppx_object() {
    return EppxObject();
}

// Repr function
std::string eppx_repr(const eppx_variant& obj) {
    return std::visit([](const auto& value) -> std::string {
        using T = std::decay_t<decltype(value)>;
        if constexpr (std::is_same_v<T, std::string>) {
            return "'" + value + "'";
        } else if constexpr (std::is_same_v<T, long long>) {
            return std::to_string(value);
        } else if constexpr (std::is_same_v<T, double>) {
            return std::to_string(value);
        } else if constexpr (std::is_same_v<T, bool>) {
            return value ? "True" : "False";
        } else if constexpr (std::is_same_v<T, std::vector<eppx_variant>>) {
            std::string result = "[";
            for (size_t i = 0; i < value.size(); ++i) {
                if (i > 0) result += ", ";
                result += eppx_repr(value[i]);
            }
            result += "]";
            return result;
        } else {
            return "<object>";
        }
    }, obj);
}

// Slice class
class EppxSlice {
private:
    std::optional<long long> start_val;
    std::optional<long long> stop_val;
    std::optional<long long> step_val;

public:
    EppxSlice(std::optional<long long> start = std::nullopt, 
              std::optional<long long> stop = std::nullopt, 
              std::optional<long long> step = std::nullopt)
        : start_val(start), stop_val(stop), step_val(step) {}
    
    std::optional<long long> start() const { return start_val; }
    std::optional<long long> stop() const { return stop_val; }
    std::optional<long long> step() const { return step_val; }
    
    std::string to_string() const {
        std::string result = "slice(";
        if (start_val) result += std::to_string(*start_val);
        else result += "None";
        result += ", ";
        if (stop_val) result += std::to_string(*stop_val);
        else result += "None";
        result += ", ";
        if (step_val) result += std::to_string(*step_val);
        else result += "None";
        result += ")";
        return result;
    }
};

EppxSlice eppx_slice(std::optional<long long> start = std::nullopt, 
                     std::optional<long long> stop = std::nullopt, 
                     std::optional<long long> step = std::nullopt) {
    return EppxSlice(start, stop, step);
}

// Vars function
std::map<std::string, eppx_variant> eppx_vars(const eppx_variant& obj = eppx_variant{}) {
    // Simplified implementation - would need object introspection
    return std::map<std::string, eppx_variant>();
}

// Improved zip function
template<typename Container1, typename Container2>
std::vector<std::pair<typename Container1::value_type, typename Container2::value_type>> 
eppx_zip(const Container1& c1, const Container2& c2) {
    using Value1 = typename Container1::value_type;
    using Value2 = typename Container2::value_type;
    std::vector<std::pair<Value1, Value2>> result;
    
    auto it1 = c1.begin();
    auto it2 = c2.begin();
    
    while (it1 != c1.end() && it2 != c2.end()) {
        result.emplace_back(*it1, *it2);
        ++it1;
        ++it2;
    }
    
    return result;
}

// Three-container zip
template<typename Container1, typename Container2, typename Container3>
std::vector<std::tuple<typename Container1::value_type, 
                      typename Container2::value_type, 
                      typename Container3::value_type>> 
eppx_zip(const Container1& c1, const Container2& c2, const Container3& c3) {
    using Value1 = typename Container1::value_type;
    using Value2 = typename Container2::value_type;
    using Value3 = typename Container3::value_type;
    std::vector<std::tuple<Value1, Value2, Value3>> result;
    
    auto it1 = c1.begin();
    auto it2 = c2.begin();
    auto it3 = c3.begin();
    
    while (it1 != c1.end() && it2 != c2.end() && it3 != c3.end()) {
        result.emplace_back(*it1, *it2, *it3);
        ++it1;
        ++it2;
        ++it3;
    }
    
    return result;
}

// Import function (stub)
eppx_variant eppx_import(const std::string& module_name) {
    // Stub implementation - would need full module system
    throw std::runtime_error("__import__() not implemented");
}

// Character conversion functions (chr and ord)
std::string eppx_chr(long long code) {
    if (code < 0 || code > 1114111) {
        throw std::runtime_error("chr() arg not in range(0x110000)");
    }
    return std::string(1, static_cast<char>(code));
}

long long eppx_ord(const std::string& char_str) {
    if (char_str.length() != 1) {
        throw std::runtime_error("ord() expected a character, but string of length " + 
                                std::to_string(char_str.length()) + " found");
    }
    return static_cast<long long>(static_cast<unsigned char>(char_str[0]));
}

// Absolute value functions
template<typename T>
auto eppx_abs(T value) -> T {
    if constexpr (std::is_arithmetic_v<T>) {
        return std::abs(value);
    } else {
        return value; // For non-arithmetic types, return as-is
    }
}

// Specialized abs for eppx_variant
eppx_variant eppx_abs(const eppx_variant& value) {
    return std::visit([](const auto& v) -> eppx_variant {
        using T = std::decay_t<decltype(v)>;
        if constexpr (std::is_same_v<T, long long>) {
            return std::abs(v);
        } else if constexpr (std::is_same_v<T, double>) {
            return std::abs(v);
        } else {
            return v; // Return unchanged for non-numeric types
        }
    }, value);
}

// Round function with precision
double eppx_round(double value, int ndigits = 0) {
    double factor = std::pow(10.0, ndigits);
    return std::round(value * factor) / factor;
}

eppx_variant eppx_round(const eppx_variant& value, int ndigits = 0) {
    double val = variant_to_double(value);
    return eppx_round(val, ndigits);
}

// Power function with modulus
template<typename T>
T eppx_pow(T base, T exponent) {
    return std::pow(base, exponent);
}

template<typename T>
T eppx_pow(T base, T exponent, T modulus) {
    // For integer types, use modular exponentiation
    if constexpr (std::is_integral_v<T>) {
        T result = 1;
        base = base % modulus;
        while (exponent > 0) {
            if (exponent % 2 == 1) {
                result = (result * base) % modulus;
            }
            exponent = exponent >> 1;
            base = (base * base) % modulus;
        }
        return result;
    } else {
        return std::fmod(std::pow(base, exponent), modulus);
    }
}

// Additional missing functions that need implementation

// Classmethod decorator (simplified stub)
template<typename Func>
class EppxClassMethod {
private:
    Func func;
public:
    EppxClassMethod(Func f) : func(f) {}
    // In a real implementation, this would bind to the class
};

template<typename Func>
EppxClassMethod<Func> eppx_classmethod(Func func) {
    return EppxClassMethod<Func>(func);
}

// Staticmethod decorator (simplified stub)
template<typename Func>
class EppxStaticMethod {
private:
    Func func;
public:
    EppxStaticMethod(Func f) : func(f) {}
    // In a real implementation, this would be a static method
};

template<typename Func>
EppxStaticMethod<Func> eppx_staticmethod(Func func) {
    return EppxStaticMethod<Func>(func);
}

// Property decorator (simplified stub)
template<typename Getter, typename Setter = void*>
class EppxProperty {
private:
    Getter getter;
    Setter setter;
public:
    EppxProperty(Getter g, Setter s = nullptr) : getter(g), setter(s) {}
    // In a real implementation, this would create a property descriptor
};

template<typename Getter>
EppxProperty<Getter> eppx_property(Getter getter) {
    return EppxProperty<Getter>(getter);
}

// Super function (simplified stub)
class EppxSuper {
public:
    EppxSuper() = default;
    // In a real implementation, this would provide access to parent class methods
};

EppxSuper eppx_super() {
    return EppxSuper();
}

// Compile function (stub)
class EppxCodeObject {
private:
    std::string source_code;
    std::string filename;
    std::string mode;
public:
    EppxCodeObject(const std::string& source, const std::string& file, const std::string& compile_mode)
        : source_code(source), filename(file), mode(compile_mode) {}
    
    std::string get_source() const { return source_code; }
    std::string get_filename() const { return filename; }
    std::string get_mode() const { return mode; }
};

EppxCodeObject eppx_compile(const std::string& source, const std::string& filename, const std::string& mode) {
    // Stub implementation - would need full compiler
    return EppxCodeObject(source, filename, mode);
}

// Output operators for new classes (defined after class definitions)
std::ostream& operator<<(std::ostream& os, const EppxByteArray& ba) {
    os << "bytearray(b'" << ba.to_string() << "')";
    return os;
}

std::ostream& operator<<(std::ostream& os, const EppxBytes& b) {
    os << "b'" << b.to_string() << "'";
    return os;
}

std::ostream& operator<<(std::ostream& os, const EppxObject& obj) {
    os << obj.to_string();
    return os;
}

std::ostream& operator<<(std::ostream& os, const EppxSlice& s) {
    os << s.to_string();
    return os;
}

std::ostream& operator<<(std::ostream& os, const EppxMemoryView& mv) {
    os << "<memory at 0x" << std::hex << reinterpret_cast<uintptr_t>(mv.data()) << ">";
    return os;
}

#endif // EPPX_BUILTINS_HPP
