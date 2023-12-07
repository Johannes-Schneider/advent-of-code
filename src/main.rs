use clap::{arg, ArgMatches, Command};

mod day1;
mod day2;
mod day3;

fn cli() -> Command {
    Command::new("aoc")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("day1")
                .arg(arg!(<FILE> "The input file for the challenge."))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("day2")
                .arg(arg!(<FILE> "The input file for the challenge."))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("day3")
                .arg(arg!(<FILE> "The input file for the challenge."))
                .arg_required_else_help(true),
        )
}

fn day1(sub_matches: &ArgMatches) {
    let file = sub_matches.get_one::<String>("FILE").expect("required arg");
    let maybe_result = day1::day1(file);
    println!("Result of Day1 - Challenge 1: {}", maybe_result.unwrap());
}

fn day2(sub_matches: &ArgMatches) {
    let file = sub_matches.get_one::<String>("FILE").expect("required arg");
    let challenge1_result = day2::day2_challenge1(file).unwrap();
    let challenge2_result = day2::day2_challenge2(file).unwrap();
    println!("Result of Day2 - Challenge 1: {}", challenge1_result);
    println!("Result of Dat2 - Challenge 2: {}", challenge2_result);
}

fn day3(sub_matches: &ArgMatches) {
    let file = sub_matches.get_one::<String>("FILE").expect("required arg");
    let challenge1_result = day3::day3_challenge1(file).unwrap();

    println!("Result of Day 3 - Challenge 1: {}", challenge1_result);
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("day1", sub_matches)) => day1(sub_matches),
        Some(("day2", sub_matches)) => day2(sub_matches),
        Some(("day3", sub_matches)) => day3(sub_matches),
        _ => unreachable!(),
    }
}
