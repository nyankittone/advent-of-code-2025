use aoc_2025 as aoc;

fn joltage_part_1(input: &[u8]) -> i64 {
    let mut max_joltage = 0i64;

    for i in 0..(input.len() - 1) {
        for ii in (i + 1)..input.len() {
            let joltage = ((input[i] - ('0' as u8)) * 10 + (input[ii] - ('0' as u8))) as i64;
            max_joltage = if joltage > max_joltage { joltage } else { max_joltage };
        }
    }

    max_joltage
}

fn find_the_big_guy(slice: &[u8]) -> Option<usize> {
    for digit in (1..=9).map(|x| x + ('0' as u8)).rev() {
        if let Some(index) = slice.iter().position(|x| *x == digit) {
            return Some(index);
        }
    }

    None
}

fn joltage_part_2(line: &[u8]) -> i64 {
    let mut returned = 0i64;
    let mut left_ptr: usize = 0;
    let mut right_ptr: usize = line.len() - 11;

    for _ in 0..12 {
        let subslice = &line[left_ptr..right_ptr];

        let i = find_the_big_guy(subslice).unwrap();
        returned = returned * 10 + (subslice[i] - ('0' as u8)) as i64;

        left_ptr += i + 1;
        right_ptr += 1;
    }

    returned
}

pub fn enter(input: &aoc::DayInput) -> i64 {
    let mut sum = 0i64;

    input.text.lines().for_each(|line| {
        if let aoc::DayPart::PartOne = input.day_part {
            sum += joltage_part_1(line.as_bytes());
        } else {
            sum += joltage_part_2(line.as_bytes());
        }
    });

    sum
}

