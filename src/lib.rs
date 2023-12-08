use clap::ArgMatches;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

mod day1;
mod day2;
mod day3;

pub struct GenericError {
    message: &'static str,
}

impl GenericError {
    pub fn new(message: &'static str) -> GenericError {
        GenericError { message }
    }

    pub fn not_implemented() -> GenericError {
        GenericError {
            message: "not implemented",
        }
    }
}

impl Error for GenericError {}

impl Display for GenericError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.message)
    }
}

impl Debug for GenericError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "An error occurred: {}", self.message)
    }
}

pub fn day1(sub_matches: &ArgMatches) {
    let file = sub_matches.get_one::<String>("FILE").expect("required arg");
    let maybe_result = day1::day1(file);
    println!("Result of Day1 - Challenge 1: {}", maybe_result.unwrap());
}

pub fn day2(sub_matches: &ArgMatches) {
    let file = sub_matches.get_one::<String>("FILE").expect("required arg");
    let challenge1_result = day2::day2_challenge1(file).unwrap();
    let challenge2_result = day2::day2_challenge2(file).unwrap();
    println!("Result of Day2 - Challenge 1: {}", challenge1_result);
    println!("Result of Dat2 - Challenge 2: {}", challenge2_result);
}

pub fn day3(sub_matches: &ArgMatches) {
    let file = sub_matches.get_one::<String>("FILE").expect("required arg");
    let challenge1_result = day3::day3_challenge1(file).unwrap();
    let challenge2_result = day3::day3_challenge2(file).unwrap();

    println!("Result of Day 3 - Challenge 1: {}", challenge1_result);
    println!("Result of Day 3 - Challenge 3: {}", challenge2_result);
}

#[cfg(test)]
mod tests {}
