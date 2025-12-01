use aoc_2025 as aoc;

#[derive(Copy ,Clone)]
struct CycleNumber(i32);

impl CycleNumber {
    fn new() -> Self {
        Self(50)
    }

    fn shift(&mut self, amount: i32) -> u32 {
        if amount == 0 {
            return 0;
        }

        let mut returned: u32 = 0;
        returned += (if amount < 0 {
            -amount
        } else {
            amount
        } / 100) as u32;

        let old = self.0;

        let mut tmp_result = self.0 + (amount % 100);

        // could pattern matching be used here?
        if tmp_result > 99 {
            returned += 1;
            tmp_result -= 100;
        } else if tmp_result < 0 {
            if old != 0{
            returned += 1;
            }
            tmp_result += 100;
        } else if tmp_result == 0 {
            returned += 1;
        }

        self.0 = tmp_result;

        returned
    }

    fn get(self) -> i32 {
        self.0
    }
}

pub fn enter(input: &aoc::DayInput) -> i64 {
    let mut spinny = CycleNumber::new();
    let mut returned: i64 = 0;

    if let aoc::DayPart::PartOne = input.day_part {
        for line in input.text.to_lowercase().lines() {
            let mut line = line.chars();
            if let Some(first) = line.next() {
                spinny.shift(line.as_str().parse::<i32>().unwrap() * (if first == 'l' {-1} else {1}));
                if spinny.get() == 0 {
                    returned += 1;
                }
            } else {
                panic!("lmao");
            }
        }
    } else {
        for line in input.text.to_lowercase().lines() {
            let mut line = line.chars();
            let first = line.next().unwrap();
            returned += spinny.shift(line.as_str().parse::<i32>().unwrap() * (if first == 'l' {-1} else {1})) as i64;
        }
    }

    returned
}

