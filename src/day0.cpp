#include <algorithm>
#include <cstring>
#include <numeric>
#include <string>
#include <ranges>

#include "common.hpp"

// Things to do in C++:
// Make common.hpp a module
// Learn the string functions that exist in string_view and string
// Learn how iterators work and their interfaces
// std::unique_ptr

template <std::size_t SIZE>
class CalorieCollection {
    private:
        std::array<std::int64_t, SIZE> data;

    public:
        CalorieCollection() {
            std::fill(data.begin(), data.end(), 0);
        };

        void addRecord(const std::int64_t record) {
            for(auto i = 0; i < SIZE; i++) {
                if(record > data[i]) {
                    std::memmove(data.data() + i + 1, data.data() + i, sizeof(std::int64_t) * (SIZE - i - 1));
                    data[i] = record;
                    return;
                }
            }
        }

        std::int64_t sumRecords() {
            return std::accumulate(data.begin(), data.end(), 0);
        }
};

template <std::size_t SIZE>
std::int64_t gimmeAnswerPls(const std::string_view &input) {
    CalorieCollection<SIZE> calories;
    std::int64_t current_calories = 0;

    for(auto line : std::ranges::split_view(input, '\n')) {
        std::string_view line_view(line);

        if(line_view == "") {
            calories.addRecord(current_calories);
            current_calories = 0;

            continue;
        }

        current_calories += std::stoi(line_view.data());
    }

    return calories.sumRecords();
}

enter(day0) {
    std::string_view input_repr(input->text, input->text_length);
    return input->day_part == aoc::DayPart::DayOne ?
        gimmeAnswerPls<1>(input_repr) : gimmeAnswerPls<3>(input_repr); 
}

