#include <cstdint>
#include <ranges>
#include <set>
#include <vector>
#include <algorithm>

#include "common.hpp"

// I REALLY need to understand what std::set and std::unordered_set actually do. I was trying to use
// them for ages only to get incredibly confused. Used a vector instead and immediately got the
// right answer for part 2. grrrrrr

class XY {
    public:
        std::uint64_t x, y;

        XY(const std::uint64_t x, const std::uint64_t y) {
            this->x = x;
            this->y = y;
        }

        XY() {
            this->x = 0;
            this->y = 0;
        }
};

bool operator==(XY const& thing1, XY const& thing2) {
    return thing1.x == thing2.x && thing1.y == thing2.y;
}

// What exactly does it mean for one XY to be "less than" another XY????
bool operator<(XY const& thing1, XY const& thing2) {
    return (thing1.x < thing2.x && thing1.y <= thing2.y) || (thing1.x <= thing2.x && thing1.y < thing2.y);
}

class Rolls {
    private:
        std::string_view data;
        std::vector<XY> removed;
        XY shape;

    public:
        Rolls(const char *const ptr, const std::size_t length) {
            data = std::string_view{ptr, length};
            
            std::uint64_t x = data.find_first_of('\n');
            std::uint64_t y = 0;
            for(auto meow : std::ranges::split_view(data, '\n')) {
                if(meow.size() > 0) y++;
            }

            shape = XY{x, y};
        }

        XY getShape() {
            return shape;
        }

        std::uint64_t getAdjacentRolls(std::int64_t x, std::int64_t y) {
            std::uint64_t sum = 0;

            auto checkOffset = [this, x, y, &sum](int offset_x, int offset_y) {
                if(this->isRoll(x + offset_x, y + offset_y)) {
                    sum++;
                }
            };

            checkOffset(-1, -1);
            checkOffset(0, -1);
            checkOffset(1, -1);
            checkOffset(-1, 0);
            checkOffset(1, 0);
            checkOffset(-1, 1);
            checkOffset(0, 1);
            checkOffset(1, 1);

            return sum;
        }

        bool isRoll(std::int64_t x, std::int64_t y) {
            auto index = ((shape.x + 1) * y) + x;
            // std::println("meow {}", removed.size());

            try {
                // if(removed.swap (XY(x, y))) return false;
                if(std::find(removed.begin(), removed.end(), XY(x, y)) != removed.end()) {
                    return false;
                }

                return data[index] == '@' ? true:false;
            } catch(std::out_of_range &e) {
                return false;
            }
        }

        void remove(std::int64_t x, std::int64_t y) {
            // removed.insert(XY(x ,y));
            removed.push_back(XY(x, y));
        }
};

std::int64_t part1(Rolls &rolls) {
    std::int64_t returned = 0;
    auto shape = rolls.getShape();

    for(std::uint64_t y = 0; y < shape.y; y++) {
        for(std::uint64_t x = 0; x < shape.x; x++) {
            if(rolls.isRoll(x, y) && rolls.getAdjacentRolls(x, y) < 4) {
                returned++;
            }
        }
    }

    return returned;
}

std::int64_t part2(Rolls &rolls) {
    std::int64_t returned = 0;
    auto shape = rolls.getShape();

    while(true) {
        std::set<XY> removed{};

        for(uint64_t y = 0; y < shape.y; y++) {
            for(uint64_t x = 0; x < shape.x; x++) {
                if(rolls.isRoll(x, y) && rolls.getAdjacentRolls(x, y) < 4) {
                    // std::println("match!");
                    returned++;
                    removed.insert(XY(x, y));
                    rolls.remove(x, y);
                }
            }
        }

        if(removed.size() == 0) break;
    }

    return returned;
}

enter(day4) {
    Rolls rolls(input->text, input->text_length);
    return (input->day_part == aoc::DayPart::PartOne ? &part1 : &part2)(rolls);
}

