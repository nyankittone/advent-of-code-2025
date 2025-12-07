#include <print>
#include <iterator>
#include <ranges>
#include <string>
#include <vector>
#include <sstream>
#include <iterator>
#include <optional>

#include "common.hpp"

// I really ought to learn C++ properly before doing each of these days. Really. Same with Rust.

enum class Operator {
    None,
    Add,
    Mult,
};

class FunnyMath {
    private:
        std::string_view slice;

        std::vector<std::size_t> cursors;
        Operator op;

        std::size_t init_last_cursor;

        char moveCursors() {
            if(slice[cursors[0]] == '\n') return '\n';

            for(std::size_t i = 0; i < cursors.size(); i++) {
                cursors[i] += 1;
            }

            if(slice[cursors[0]] == '\n') return '\n';

            if([this](){
                for(std::size_t i = 0; i < this->cursors.size() - 1; i++) {
                    if(this->slice[this->cursors[i]] != ' ') {
                        return false;
                    }
                }

                return true;
            }()) {
                return ' ';
            }

            switch(slice[cursors[cursors.size() - 1]]) {
                case '*':
                    return '*';
                case '+':
                    return '+';
                default:
                    break;
            }

            return '\0';
        }

    public:
        FunnyMath(std::string_view text) {
            // we'll just call std::find in a loop hahahahahahahahahahahaha
            // ok so using the C++ iterators honestly wasn't *that* bad... :)
            slice = text;
            cursors.push_back(0);
            for(auto iter = slice.begin(); (iter = std::find(iter, slice.end(), '\n')) != slice.end(); ++iter) {
                if(iter + 1 == slice.end()) {
                    break;
                }

                cursors.push_back(std::distance(slice.begin(), iter + 1));
            }

            op = Operator::None;
            init_last_cursor = cursors[cursors.size() - 1];

            std::println("{}", cursors);
        }

        std::optional<std::int64_t> yield() {
            // moveCursors() until we get operators or a newline
            char move_result = 0;
            if(cursors[cursors.size() - 1] > init_last_cursor) {
                while((move_result = moveCursors()) == '\0');
            } else {
                switch(slice[cursors[cursors.size() - 1]]) {
                    case '+':
                        op = Operator::Add;
                        break;
                    case '*':
                        op = Operator::Mult;
                        break;
                }
            }

            switch(move_result) {
                case '\n':
                    return std::nullopt;
                case '+':
                    op = Operator::Add;
                    break;
                case '*':
                    op = Operator::Mult;
                    break;
                case '\0':
                    break;
                default:
                    std::println("AAAAA!");
                    throw "fuck you";
            }

            // operatees.clear();

            std::int64_t returned = op == Operator::Add ? 0 : 1;

            // Loop through every digit until encountering a column divider
            do {
                std::int64_t number = 0;

                for(std::size_t i = 0; i < cursors.size() - 1; i++) {
                    std::int64_t character = slice[cursors[i]];
                    if(character >= '0' && character <= '9') {
                        number = number * 10 + character - '0';
                    }
                }

                std::println("Number fetched: {}", number);

                if(op == Operator::Add) {
                    returned += number;
                } else {
                    returned *= number;
                }
            } while((move_result = moveCursors()) == '\0');

            std::println("Returning {}", returned);

            return std::optional<std::int64_t>(returned);
        }
};

std::int64_t solvePart2(std::string_view text) {
    // Meow :3
    // create object for iterating over each column in the text
    // iterate over each column scanned, summing stuff up
    // return the sum

    FunnyMath funny(text);
    std::int64_t returned = 0;
    
    for(std::optional<std::int64_t> result; (result = funny.yield()) != std::nullopt;) {
        returned += result.value();
    }

    return returned;
}

enter(day6) {
    std::string_view text(input->text, input->text_length);
    std::vector<std::vector<std::string_view>> bullshit{};

    if(input->day_part == aoc::DayPart::PartTwo) {
        return solvePart2(text);
    }

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

