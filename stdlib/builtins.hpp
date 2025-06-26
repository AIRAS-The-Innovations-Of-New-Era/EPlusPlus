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
auto eppx_min(const Container& container) -> typename Container::value_type {
    return container.empty() ? typename Container::value_type{} : *std::min_element(container.begin(), container.end());
}

template<typename Container>
auto eppx_max(const Container& container) -> typename Container::value_type {
    return container.empty() ? typename Container::value_type{} : *std::max_element(container.begin(), container.end());
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

// Context manager support for with statements
template<typename FileType>
class EppxFileContextManager {
private:
    FileType file_obj;
    bool should_close;

public:
    EppxFileContextManager(FileType f) : file_obj(f), should_close(true) {}

    FileType& __enter__() {
        return file_obj;
    }

    bool __exit__(const std::string& exc_type = "", 
                  const std::string& exc_val = "", 
                  const std::string& exc_tb = "") {
        if (should_close && file_obj) {
            file_obj->close();
        }
        return false; // Don't suppress exceptions
    }
};

// Helper function to create context manager
template<typename FileType>
EppxFileContextManager<FileType> eppx_with_file(FileType file_obj) {
    return EppxFileContextManager<FileType>(file_obj);
}

#endif // EPPX_BUILTINS_HPP
