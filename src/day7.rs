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

fn part2_rec<'a, I>(mut lines: I, beam_index: isize, width: usize) -> i64
where I: Clone + Iterator<Item = &'a str> {
    if beam_index < 0 || beam_index >= (width as isize) {
        return 1;
    }

    while let Some(line) = lines.next() {
        if line.as_bytes()[beam_index as usize] == '^' as u8 {
            return part2_rec(lines.clone(), beam_index - 1, width) +
                part2_rec(lines.clone(), beam_index + 1, width);
        }
    }

    1
}

fn part2<'a, I: Clone + Iterator<Item = &'a str>>(mut lines: I) -> i64 {
    let first_line = lines.next().unwrap();

    // let beam_index: isize = first_line.chars().position(|x| x == 'S').unwrap().try_into().unwrap();
    let beam_index: isize = first_line.bytes().position(|x| x == 'S' as u8).unwrap().try_into().unwrap();
    let width: usize = first_line.chars().count();

    part2_rec(lines.clone(), beam_index, width)
}

pub fn enter(input: &aoc::DayInput) -> i64 {
    (match input.day_part {
        aoc::DayPart::PartOne => part1,
        aoc::DayPart::PartTwo => part2,
    })(input.text.lines().enumerate().filter(|x| x.0 % 2 == 0).map(|x| x.1))
}

