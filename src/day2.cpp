#include <string>
#include <ranges>
#include <print>
#include <string_view>
#include <cmath>

#include "common.hpp"

// 2025, day 2
// This solution is *really* convincing me that I should understand C++ iterators more. Also a lot
// of this solution is quite C-like, which I suppose is not bad? But the point of this is to learn
// more about C++ and Rust.

bool isValid1(std::int64_t n) {
    // get length with log10
    // "split" number into left and right, return compare result

    std::int64_t length = std::floor(std::log10(n)) + 1;
    if(length % 2 == 1) return true;

    std::int64_t right = n % (static_cast<std::int64_t>(std::pow(10, length / 2)));
    std::int64_t left = n / (static_cast<std::int64_t>(std::pow(10, length / 2)));

    if(left == right) {
        return false;
    }

    return true;
}

bool isValid2(std::int64_t n) {
    std::int64_t length = std::floor(std::log10(n)) + 1;

    // sigh, what if I just loop over trying to check for a pattern one-wide, then two-wide, then
    // three, etc? Fuck performance, my computer is goated with the sauce anyway
    for(std::int64_t size = 1; size <= length / 2; size++) {
        if(length % size) continue;

        std::int64_t divide_by = std::pow(10, size);

        for(std::int64_t operate_n = n, mod_result = n % divide_by; operate_n > 0; operate_n /= divide_by) {
            if((operate_n % divide_by) != mod_result) {
                goto continue_upper;
            }
        }

        return false;
        continue_upper:
    }

    return true;
}

std::int64_t sumInvalid(bool (*valid)(std::int64_t), std::int64_t begin, std::int64_t end) {
    std::int64_t sum{};

    // Let's jsut do this the stupid way...
    for(std::int64_t current = begin; current <= end; current++) {
        if(!valid(current)) sum += current;
    }

    return sum;
}

enter(day2) {
    std::int64_t returned = 0;
    std::string joined_input(input->text, input->text_length);
    std::erase(joined_input, '\n');

    for(auto field : std::ranges::split_view(joined_input, ',')) {
        size_t left_length = 0;

        for(; left_length < field.size(); left_length++) {
            if(field[left_length] == '-') break;
        }

        std::string_view left(field.data(), left_length);
        std::string_view right(field.data() + left_length + 1, field.size() - left_length);

        returned += sumInvalid (
            input->day_part == aoc::DayPart::PartOne ? &isValid1 : &isValid2,
            std::stol(left.data()),
            std::stol(right.data())
        );
    }

    return returned;
}

