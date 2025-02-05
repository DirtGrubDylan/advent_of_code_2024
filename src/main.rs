pub mod util;

mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_2;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
mod day_25;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

use std::io::{self, Write};

fn print_seperator() {
    println!("-------------------------------------");
}

fn run_day(day: u32) {
    match day {
        1 => day_1::run(),
        2 => day_2::run(),
        3 => day_3::run(),
        4 => day_4::run(),
        5 => day_5::run(),
        6 => day_6::run(),
        7 => day_7::run(),
        8 => day_8::run(),
        9 => day_9::run(),
        10 => day_10::run(),
        11 => day_11::run(),
        12 => day_12::run(),
        13 => day_13::run(),
        14 => day_14::run(),
        15 => day_15::run(),
        16 => day_16::run(),
        17 => day_17::run(),
        18 => day_18::run(),
        19 => day_19::run(),
        20 => day_20::run(),
        21 => day_21::run(),
        22 => day_22::run(),
        23 => day_23::run(),
        24 => day_24::run(),
        25 => day_25::run(),
        _ => unimplemented!("I haven't done that day yet :("),
    }
}

/// Gets the user input.
///
/// # Panics
///
/// If line couldn't be flushed and/or stdin couldn't be read/parsed.
#[must_use]
pub fn get_user_input() -> u32 {
    let mut input_buffer = String::new();

    io::stdout().flush().expect("Could not flush stdout!");

    io::stdin()
        .read_line(&mut input_buffer)
        .expect("Failed to read user input!");

    input_buffer
        .trim()
        .parse::<u32>()
        .expect("Failed to parse user_input!")
}

fn main() {
    print_seperator();

    print!("Please choose a day to run (1-25): ");

    let input = get_user_input();

    print_seperator();

    run_day(input);

    print_seperator();
}
