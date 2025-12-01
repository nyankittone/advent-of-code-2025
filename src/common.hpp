#include <cstdint>

namespace aoc {
    enum class DayPart {
        DayOne,
        DayTwo,
    };

    extern "C" struct DayInput {
        const char *text;
        const std::size_t text_length;
        const DayPart day_part;
    };
};

#define enter(function_name) \
    extern "C" std::int64_t function_name(const aoc::DayInput *const input)

