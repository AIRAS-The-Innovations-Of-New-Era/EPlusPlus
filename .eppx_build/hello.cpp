#include <iostream>
#include <string>

void eppx_print(const std::string& s) {
    std::cout << s << std::endl;
}

int main() {
    eppx_print("Hello, E++ World!");
    eppx_print("This is E++ running natively!");
    return 0;
}
