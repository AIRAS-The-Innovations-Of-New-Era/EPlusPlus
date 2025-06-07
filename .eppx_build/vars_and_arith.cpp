#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <cmath> // Added for std::pow

void eppx_print(const std::string& s) { std::cout << s << std::endl; }
void eppx_print(long long x) { std::cout << x << std::endl; }
void eppx_print(int x) { std::cout << x << std::endl; }
void eppx_print(bool b) { std::cout << (b ? "true" : "false") << std::endl; }

int main() {
    long long x;
    x = 42LL;
    eppx_print(x);
    eppx_print(123LL);
    long long y;
    y = (x + 5LL);
    eppx_print(y);
    eppx_print((1LL + (2LL * 3LL)));
    eppx_print(std::string("yo welcome from e++"));
    eppx_print((10LL % 3LL));
    eppx_print(static_cast<long long>(std::pow(2LL, 4LL)));
    eppx_print((10LL / 3LL));
    eppx_print((11LL / 3LL));
    eppx_print((static_cast<long long>(std::pow(5LL, 2LL)) % 3LL));
    eppx_print((2LL * static_cast<long long>(std::pow(3LL, 2LL))));
    eppx_print((10LL == 10LL));
    eppx_print((10LL == 5LL));
    eppx_print((10LL != 5LL));
    eppx_print((10LL != 10LL));
    eppx_print((10LL > 5LL));
    eppx_print((5LL > 10LL));
    eppx_print((10LL < 20LL));
    eppx_print((20LL < 10LL));
    eppx_print((10LL >= 10LL));
    eppx_print((10LL >= 11LL));
    eppx_print((10LL <= 10LL));
    eppx_print((10LL <= 9LL));
    eppx_print((x == 42LL));
    eppx_print((y != (x + 5LL)));
    long long a;
    a = 10LL;
    eppx_print(a);
    a += 5LL;
    eppx_print(a);
    a -= 3LL;
    eppx_print(a);
    a *= 2LL;
    eppx_print(a);
    a /= 4LL;
    eppx_print(a);
    long long b;
    b = 25LL;
    b %= 4LL;
    eppx_print(b);
    long long c;
    c = 2LL;
    c = static_cast<long long>(std::pow(c, 3LL));
    eppx_print(c);
    long long d;
    d = 17LL;
    d /= 5LL;
    eppx_print(d);
    eppx_print(((1LL) && (0LL)));
    eppx_print(((1LL) && (1LL)));
    eppx_print(((0LL) && (0LL)));
    eppx_print(((1LL) || (0LL)));
    eppx_print(((0LL) || (1LL)));
    eppx_print(((0LL) || (0LL)));
    eppx_print(!(1LL));
    eppx_print(!(0LL));
    eppx_print((((x == 42LL)) && ((y == 47LL))));
    eppx_print((((x == 42LL)) || ((y == 0LL))));
    eppx_print(!((x == 0LL)));
    eppx_print((6LL & 3LL));
    eppx_print((6LL | 3LL));
    eppx_print((6LL ^ 3LL));
    eppx_print(~(6LL));
    eppx_print((3LL << 2LL));
    eppx_print((12LL >> 2LL));
    long long val1;
    val1 = 100LL;
    long long val2;
    val2 = 100LL;
    eppx_print((val1 == val2) /* Placeholder for IS */);
    eppx_print((val1 != val2) /* Placeholder for IS NOT */);
    std::string str_a;
    str_a = std::string("hello world");
    std::string str_b;
    str_b = std::string("world");
    std::string str_c;
    str_c = std::string("earth");
    eppx_print((str_a.find(str_b) != std::string::npos) /* Placeholder for IN */);
    eppx_print((str_a.find(str_c) != std::string::npos) /* Placeholder for IN */);
    eppx_print((str_a.find(str_b) == std::string::npos) /* Placeholder for NOT IN */);
    eppx_print((str_a.find(str_c) == std::string::npos) /* Placeholder for NOT IN */);
    long long bit_val;
    bit_val = 6LL;
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
