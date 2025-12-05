use std::ops::Deref;

use aoc_2025 as aoc;

#[derive(Debug)]
struct FoodRanges {
    ranges: Vec<(i64, i64)>,
}

impl FoodRanges {
    fn new(input: &str) -> Self {
        let mut returned = Self {ranges: Vec::with_capacity(1337)};

        for line in input.lines() {
            let mut line_iter = line.split("-");
            let left = line_iter.next().unwrap().parse::<i64>().unwrap();
            let right = line_iter.next().unwrap().parse::<i64>().unwrap();

            returned.ranges.push(if left > right {
                (right, left)
            } else {
                (left, right)
            });
        }

        returned.simplify();
        returned
    }

    fn simplify(&mut self) {
        // sort ranges by lower bound
        // start merging shit

        self.ranges.sort_by(|x, y| x.0.cmp(&y.0));

        let mut ranges_length = self.ranges.len();
        let mut i = 0usize;
        while i < ranges_length - 1 {
            // if two adjacent ranges overlap, delete one of them and make a merged one in-place
            // A Vec<T> is not an ideal data type for this at all. But whatever.
            if self.ranges[i].1 < self.ranges[i + 1].0 {
                i += 1;
                continue;
            }

            let new_upper: i64 = if self.ranges[i].1 > self.ranges[i + 1].1 {
                self.ranges[i].1
            } else {
                self.ranges[i + 1].1
            };

            self.ranges.remove(i + 1);
            ranges_length -= 1;
            self.ranges[i].1 = new_upper;
        }
    }

    fn is_fresh(&self, id: i64) -> bool {
        for (lower, upper) in &self.ranges {
            if id >= *lower && id <= *upper {
                return true;
            }
        }

        false
    }
}

impl Deref for FoodRanges {
    type Target = Vec<(i64, i64)>;

    fn deref(&self) -> &Self::Target {
        &self.ranges
    }
}

pub fn enter(input: &aoc::DayInput) -> i64 {
    // split string by ranges and IDs
    // load ranges into an object
    // iterate over IDs

    let index = input.text.find("\n\n").unwrap();
    let ranges = FoodRanges::new(&input.text[..index]);

    if let aoc::DayPart::PartOne = input.day_part {
        let ids = &input.text[(index + 2)..];
        let mut returned = 0i64;

        for line in ids.lines() {
            let number = line.parse::<i64>().unwrap();
            if ranges.is_fresh(number) {
                returned += 1;
            }
        }

        return returned;
    }

    ranges.iter().map(|(lower, upper)| (upper - lower) + 1).sum()
}

