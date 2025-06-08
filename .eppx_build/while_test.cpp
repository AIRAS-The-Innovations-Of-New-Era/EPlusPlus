#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <cmath> // Added for std::pow

void eppx_print(const std::string& s) { std::cout << s << std::endl; }
void eppx_print(long long x) { std::cout << x << std::endl; }
void eppx_print(double x) { std::cout << x << std::endl; }
void eppx_print(int x) { std::cout << x << std::endl; }
void eppx_print(bool b) { std::cout << (b ? "true" : "false") << std::endl; }

int main() {
    long long i;
    i = 0LL;
    while ((i < 5LL)) {
    eppx_print(std::string("Loop iteration:"));
    eppx_print(i);
    i += 1LL;
    }
    eppx_print(std::string("Loop completed"));
    long long countdown;
    countdown = 10LL;
    while ((countdown > 0LL)) {
    eppx_print(std::string("Countdown:"));
    eppx_print(countdown);
    countdown -= 1LL;
    }
    eppx_print(std::string("Blast off!"));
    long long x;
    x = 1LL;
    long long y;
    y = 100LL;
    while ((((x < y)) && ((x < 50LL)))) {
    eppx_print(std::string("x is now:"));
    eppx_print(x);
    x *= 2LL;
    }
    eppx_print(std::string("While loop tests complete."));
    return 0;
}
