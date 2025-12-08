#include <algorithm>
#include <print>
#include <ranges>
#include <tuple>
#include <vector>
#include <string>
#include <regex>
#include <cmath>

#include "common.hpp"

using triplet = std::tuple<std::int64_t, std::int64_t, std::int64_t>;

class CircuitGroup {
    private:
        std::vector<std::vector<triplet>> groups;

    public:
        // I am going to concede defeat for this AoC day for now... I cannot implement this function
        // right after multiple hours. After all the AoC questions are done, I should re-analyze
        // this problem more deeply, and/or use ChatGPT to better understand the problem at hand.
        bool addPair(const triplet &x, const triplet &y) {
            // Figure out the circuit to add this pair to
            // BUG: There is something wrong with this for loop.
            for(std::vector circuit : groups) {
                // if neither first reference found in circuit nor second reference found, continue;
                if(std::find(circuit.begin(), circuit.end(), x) == circuit.end()) {
                    if(std::find(circuit.begin(), circuit.end(), y) == circuit.end()) {
                        // found none
                        std::println("FUCK");
                        continue;
                    }

                    // found Y without X
                    circuit.push_back(x);
                    return true;
                }

                if(std::find(circuit.begin(), circuit.end(), y) == circuit.end()) {
                    // found X without Y
                    circuit.push_back(y);
                    return true;
                }

                // found both
                return false;
            }

            // I promise I did not vibe-code this. I'm just bad.
            if(groups.size() > 1) {
                std::println("WOW MEOW");
            for(std::size_t i = 0; i < groups.size() - 1; i++) {
                for(std::size_t ii = i + 1; ii < groups.size(); ii++) {
                    if((std::find(groups[i].begin(), groups[i].end(), x) != groups[i].end() &&
                        std::find(groups[ii].begin(), groups[i].end(), y) != groups[ii].end()) ||
                    (std::find(groups[i].begin(), groups[i].end(), y) != groups[i].end() &&
                     std::find(groups[ii].begin(), groups[i].end(), x) != groups[ii].end())) {
                        // merge shit idfk
                        groups[i].insert(groups[i].end(), groups[ii].begin(), groups[ii].end());
                        return true;
                    }
                }
            }
            }

            // If we couldn't, make a new circuit
            groups.push_back(std::vector<triplet>{});

            groups[groups.size() - 1].push_back(x);
            groups[groups.size() - 1].push_back(y);
            return true;
        }

        std::int64_t multTop(std::size_t amount) {
            for(auto meow : groups) {
                std::println("{}", meow);
            }

            using Ass = std::vector<triplet>;
            std::sort(groups.begin(), groups.end(), [](const Ass &x, const Ass &y) -> bool {
                return x.size() > y.size();
            });

            const std::size_t goal = std::min(groups.size(), amount);
            std::int64_t returned = 1;

            for(std::size_t i = 0; i < goal; i++) {
                returned *= groups[i].size();
            }

            return returned;
        }
};

std::int64_t pythagoreanDeez(triplet params) {
    return std::sqrt(std::pow(std::get<0>(params), 2) + std::pow(std::get<1>(params), 2) + std::pow(std::get<2>(params), 2));
}

enter(day8) {
    // create vector of tuples from string_view
    std::vector<triplet> boxes{};
    for(auto line : std::ranges::split_view(std::string_view(input->text, input->text_length), '\n')) {
        std::regex re(",");
        std::string_view line_view(line.data(), line.size());

        if(line_view == "") continue;

        std::regex_token_iterator iter(line_view.begin(), line_view.end(), re, -1);

        triplet thing{};
        std::get<0>(thing) = std::stoll(*(iter++));
        std::get<1>(thing) = std::stoll(*(iter++));
        std::get<2>(thing) = std::stoll(*(iter++));

        boxes.push_back(thing);
    }

    // std::println("{}", boxes);

    // Crunch the distances of all combinations, store it in a vector
    using distance_type = std::tuple<triplet, triplet, std::int64_t>;
    std::vector<distance_type> distances{};
    distances.reserve(69420);

    for(std::size_t i = 0, boxes_count = boxes.size(); i < boxes_count - 1; i++) {
        for(std::size_t ii = i + 1; ii < boxes_count; ii++) {
            distances.push_back(std::make_tuple(boxes[i], boxes[ii], pythagoreanDeez (
                std::make_tuple(std::get<0>(boxes[i]) - std::get<0>(boxes[ii]),
                    std::get<1>(boxes[i]) - std::get<1>(boxes[ii]),
                    std::get<2>(boxes[i]) - std::get<2>(boxes[ii])
                )
            )));
        }
    }

    // Sort distances vector by distances, bottom-to-top
    std::sort(distances.begin(), distances.end(), [](const distance_type &x, const distance_type &y) -> bool {
        return std::get<2>(x) < std::get<2>(y);
    });

    std::println("{} length: {}", distances, distances.size());

    // Now what?
    // We need to start linking shit together
    // I'll delegate this to a special object lol
    CircuitGroup circuits{};
    for(auto pair : distances) {
        circuits.addPair(std::get<0>(pair), std::get<1>(pair));
    }

    std::println("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");

    return circuits.multTop(3);
}

