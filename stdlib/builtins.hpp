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

template<typename Container1, typename Container2>
auto eppx_zip(const Container1& c1, const Container2& c2) {
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

#endif // EPPX_BUILTINS_HPP
