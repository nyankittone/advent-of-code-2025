use std::collections::HashMap;
use std::ops::Deref;

use aoc_2025 as aoc;

// For part 2, a binary tree data structure makes some sense here...
// Actually, no, maybe a recursive solution makes some sense!

fn part1<'a, I: Iterator<Item = &'a str>>(mut lines: I) -> i64 {
    let mut beams = lines
        .next()
        .unwrap()
        .chars()
        .map(|x| if x == 'S' {true} else {false})
        .collect::<Vec<bool>>();

    let mut returned = 0i64;

    for line in lines {
        // get indices of each ^
        // Is it possible to merge iterators for the vec we have and this iterator
        for (i, _) in line.chars().enumerate().filter(|(_, value)| *value == '^') {
            if beams[i] == true {
                returned += 1;

                beams[i] = false;
                if let Some(value) = beams.get_mut(i - 1) {
                    *value = true;
                }

                if let Some(value) = beams.get_mut(i + 1) {
                    *value = true;
                }
            }
        }
    }

    returned
}

struct FindingMemo {
    map: HashMap<(usize, usize), i64>,
}

impl Deref for FindingMemo {
    type Target = HashMap<(usize, usize), i64>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl FindingMemo {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn put_range(&mut self, from: usize, to: usize, column: usize, data: i64) {
        for row in from..=to {
            self.map.insert((column, row), data);
        }
    }
}

fn part2_rec<'a, I>(mut lines: I, beam_index: usize, row: usize, memo: &mut FindingMemo) -> i64
where I: Clone + Iterator<Item = (usize, &'a str)> {
    if let Some(value) = memo.get(&(beam_index, row)) {
        return *value;
    }

    while let Some((current_row, line)) = lines.next() {
        if line.as_bytes()[beam_index as usize] == '^' as u8 {
            let returned = part2_rec(lines.clone(), beam_index - 1, current_row + 1, memo) +
                part2_rec(lines.clone(), beam_index + 1, current_row + 1, memo);

            memo.put_range(row, current_row, beam_index, returned);

            return returned;
        }
    }

    1
}

fn part2<'a, I: Clone + Iterator<Item = &'a str>>(mut lines: I) -> i64 {
    let first_line = lines.next().unwrap();

    let beam_index = first_line.bytes().position(|x| x == 'S' as u8).unwrap();
    let width = first_line.chars().count();

    let mut memo = FindingMemo::new();

    part2_rec(lines.enumerate().clone(), beam_index, 0, &mut memo)
}

pub fn enter(input: &aoc::DayInput) -> i64 {
    (match input.day_part {
        aoc::DayPart::PartOne => part1,
        aoc::DayPart::PartTwo => part2,
    })(input.text.lines().enumerate().filter(|x| x.0 % 2 == 0).map(|x| x.1))
}

