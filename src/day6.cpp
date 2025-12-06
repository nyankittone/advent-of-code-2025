#include <print>
#include <iterator>
#include <ranges>
#include <string>
#include <vector>
#include <sstream>
#include <iterator>

#include "common.hpp"

// I really ought to learn C++ properly before doing each of these days. Really. Same with Rust.

enter(day6) {
    std::string_view text(input->text, input->text_length);
    std::vector<std::vector<std::string_view>> bullshit{};

    for(auto line : std::ranges::split_view(text, '\n')) {
        if(line.size() == 0) break;

        bullshit.push_back(std::vector<std::string_view>{});

        std::string_view line_sv(line.data(), line.size());
        for(auto word : std::ranges::split_view(line, ' ')) {
            if(word.size() > 0) {
                bullshit[bullshit.size() - 1].push_back(std::string_view{word.data(), word.size()});
            }
        }
    }

    const std::size_t height = bullshit.size();
    const std::size_t length = bullshit[0].size();

    std::vector<std::int64_t> meow{};
    meow.reserve(height - 1);

    std::int64_t sum = 0;

    for(std::size_t x = 0; x < length; x++) {
        for(std::size_t y = 0; y < height - 1; y++) {
            meow.push_back(std::stol(bullshit[y][x].data()));
        }

        if(bullshit[height - 1][x] == "+") {
            for(std::size_t y = 0; y < height - 1; y++) {
                sum += meow[y];
            }
        } else {
            std::int64_t tmp = 1;

            for(std::size_t y = 0; y < height - 1; y++) {
                tmp *= meow[y];
            }

            sum += tmp;
        }

        meow.clear();
    }

    return sum;
}
