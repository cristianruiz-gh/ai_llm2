#include <iostream>
#include <iomanip>
#include <chrono>

int main() {
    const int ITERATIONS = 200'000'000;   // number of loop iterations
    const int P1 = 4;                     // param1
    const int P2 = 1;                     // param2

    double result = 1.0;

    auto start = std::chrono::high_resolution_clock::now();

    for (int i = 1; i <= ITERATIONS; ++i) {
        // j = i * param1 - param2
        result -= 1.0 / static_cast<double>(i * P1 - P2);
        // j = i * param1 + param2
        result += 1.0 / static_cast<double>(i * P1 + P2);
    }

    result *= 4.0;   // final multiplication

    auto end = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> elapsed = end - start;

    std::cout.setf(std::ios::fixed);
    std::cout << std::setprecision(12) << "Result: " << result << '\n';
    std::cout << std::setprecision(6)  << "Execution Time: " << elapsed.count() << " seconds\n";

    return 0;
}
