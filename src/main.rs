use std::{
    io,
    fs,
    io::Read,
};

use aoc_2025 as aoc;

// I was using a mcaro to aautogenerate all of these module headers. But rust-analyzer shits the
// bed hard if you do that... so I'm not doing that anymore. I now have to manually add a module
// and a C ABI function on each day addition... which is not ideal. It would be awesome if
// rust-analyzer was able to resolve macros to find submodules getting called, but alas.
mod day1;
mod day3;
mod day5;
mod day7;
mod day9;

unsafe extern "C-unwind" {
    fn day0(input: &aoc::CDayInput) -> i64;
    fn day2(input: &aoc::CDayInput) -> i64;
    fn day4(input: &aoc::CDayInput) -> i64;
    fn day6(input: &aoc::CDayInput) -> i64;
    fn day8(input: &aoc::CDayInput) -> i64;
}

fn main() {
    let holder = {
        let mut tmp_holder = aoc::FunctionHolder::with_capacity(32);
        tmp_holder.add_c_func(day0);
        tmp_holder.add_rust_func(day1::enter);
        tmp_holder.add_c_func(day2);
        tmp_holder.add_rust_func(day3::enter);
        tmp_holder.add_c_func(day4);
        tmp_holder.add_rust_func(day5::enter);
        tmp_holder.add_c_func(day6);
        tmp_holder.add_rust_func(day7::enter);
        tmp_holder.add_c_func(day8);
        tmp_holder.add_rust_func(day9::enter);

        tmp_holder
    };

    // parse command line arguments
    let params = aoc::ParsedArgs::new();

    // validate the day number
    if let Err(error) = &params {
        aoc::die(1, error);
    }
    let params = params.unwrap();

    // use command line arguments to generate a DayInput struct
    let file_contents: String = {
        let mut future_contents = String::new(); // I hate how I must initialize this here...
        if let Some(filename) = params.filename {
            match fs::read_to_string(&filename) {
                Ok(contents) => future_contents = contents,
                Err(error) => aoc::die(2, format!("Can't read file \"{}\": {}", filename, error)),
            }
        } else if let Err(error) = io::stdin().read_to_string(&mut future_contents) {
            aoc::die(2, format!("Can't read from stdin: {}", error));
        };

        future_contents
    };

    let day_input = aoc::DayInput::new(file_contents.as_str(), params.part);
    let day_result = holder.run_day(params.day, &day_input);

    println! (
        "Day {}, part {}: Got back {}\nTook {} μs",
        params.day,
        params.part.get_numeric(),
        day_result.result(),
        day_result.elapsed()
    );
}

