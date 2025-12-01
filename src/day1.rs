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

        // Determine clicks
        // if amount is positive, then it's just the sum of self + amount / 100
        // if negative then it's self + amount / -100 if self + amount is <= 0, else 0
        // keep in mind the case of it landing on 0, that counts too

        let old_self = self.0;
        self.0 += amount;
        let returned: u32;

        if (self.0 < 1 || self.0 > 99) && old_self != 0 {
            println!("{}", self.0);
            if amount > 0 {
                returned = (self.0 / 100) as u32;
                println!("{}: adding {}", amount, returned);
            } else {
                if self.0 > 0 {
                    returned = 0;
                } else {
                    returned = (self.0 / -100 + 1) as u32;
                    println!("{}: adding {}", amount, returned);
                }
            }
        } else {
            returned = 0;
        }

        let tmp_result = self.0 % 100;
        if tmp_result < 0 {
            self.0 = tmp_result + 100;
        } else {
            self.0 = tmp_result;
        }

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

