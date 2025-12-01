// I want a macro here for defining native and foreign functions in an array.
// some syntax like `day_count!(4)` could generate everything needed for days 0-4.
// I will need:
// a hash map defined for both native Rust and C FFI functions
// uuuuh yeah I think that's all I need lmao

use std::{
    env,
    fmt,
    ffi,
};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum DayPart {
    PartOne,
    PartTwo,
}

impl DayPart {
    pub fn get_numeric(&self) -> u8 {
        match self {
            DayPart::PartOne => 1,
            DayPart::PartTwo => 2,
        }
    }
}

#[derive(Debug)]
pub struct DayInput<'a> {
    pub text: &'a str,
    pub day_part: DayPart,
}

impl<'a> DayInput<'a> {
    pub fn new(text: &'a str, day_part: DayPart) -> Self {
        Self {
            text,
            day_part,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CDayInput {
    text: *const ffi::c_char,
    text_length: usize,
    day_part: DayPart,
}

impl CDayInput {
    pub fn from(rust_day_input: &DayInput) -> Self {
        Self {
            text: rust_day_input.text.as_ptr() as *const ffi::c_char,
            text_length: rust_day_input.text.len(),
            day_part: rust_day_input.day_part,
        }
    }
}


#[derive(Debug)]
pub struct ParsedArgs {
    pub filename: Option<String>,
    pub day: u32,
    pub part: DayPart,
}

#[derive(Debug)]
pub enum ParseError {
    NoDay,
    InvalidDay(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NoDay => write!(formatter, "no argument for day provided"),
            Self::InvalidDay(day) => write! (
                formatter, "parameter for day \"{}\" is not a valid unsigned 32-bit integer", day
            ),
        }
    }
}

impl ParsedArgs {
    fn flagless_add_file<I: Iterator<Item = String>>(mut self, iterator: &mut I) -> Result<Self, ParseError> {
        if let Some(filename) = iterator.next() {
            self.filename = Some(filename);
        }

        Ok(self)
    }

    fn flagless_add_day<I: Iterator<Item = String>>(mut self, iterator: &mut I) -> Result<Self, ParseError> {
        match iterator.next() {
            Some(arg) => match arg.parse::<u32>() {
                Ok(day) => {
                    self.day = day;
                    self.flagless_add_file(iterator)
                },
                Err(_) => Err(ParseError::InvalidDay(arg)),
            },
            None => Err(ParseError::NoDay),
        }
    }

    pub fn new() -> Result<Self, ParseError> {
        let mut args = env::args();
        if let None = args.next() {
            return Err(ParseError::NoDay);
        }

        let mut returned = Self {
            filename: None,
            day: 0,
            part: DayPart::PartOne,
        };

        let mut set_day: bool = false;

        for arg in &mut args {
            match arg.as_str() {
                "-i" => returned.part = DayPart::PartTwo,
                "--" => return returned.flagless_add_day(&mut args),
                _ => match arg.parse::<u32>() {
                    Ok(day) => {returned.day = day; set_day = true; break;},
                    Err(_) => return Err(ParseError::InvalidDay(arg)),
                },
            }
        }

        for arg in &mut args {
            match arg.as_str() {
                "-i" => returned.part = DayPart::PartTwo,
                "--" => return returned.flagless_add_file(&mut args),
                _ => {returned.filename = Some(arg); break}
            }
        }

        if !set_day {
            return Err(ParseError::NoDay);
        }

        if let Some(_) = args.find(|arg| arg == "-i") {
            returned.part = DayPart::PartTwo;
        }

        Ok(returned)
    }
}

pub fn die<T: fmt::Display>(exit_code: i32, error: T) {
    eprintln!("\x1b[1;91mfatal:\x1b[m {}!", error);
    std::process::exit(exit_code);
}

enum RustOrCFunction {
    Rust(fn(&DayInput) -> i64),
    C(unsafe extern "C-unwind" fn(&CDayInput) -> i64),
}

// I do not like how this type is implemented. Ideally, I would like for it to be a
// stack-allocated, contiguous space for the function pointers, split in half, one for the C
// functions, and the other for Rust functions. But alas, I could do something like this in C or
// C++, but not Rust. I don't know the language enough yet,,,
pub struct FunctionHolder {
    functions: Vec<RustOrCFunction>,
}

impl FunctionHolder {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
        }
    }

    pub fn with_capacity(size: usize) -> Self {
        Self {
            functions: Vec::with_capacity(size),
        }
    }

    pub fn add_rust_func(&mut self, func: fn(&DayInput) -> i64) {
        self.functions.push(RustOrCFunction::Rust(func));
    }

    pub fn add_c_func(&mut self, func: unsafe extern "C-unwind" fn(&CDayInput) -> i64) {
        self.functions.push(RustOrCFunction::C(func));
    }

    pub fn run_day(&self, day: u32, day_input: &DayInput) -> i64 {
        match self.functions[day as usize] {
            RustOrCFunction::Rust(func) => func(day_input),
            RustOrCFunction::C(func) => unsafe {func(&CDayInput::from(day_input))},
        }
    }
}

