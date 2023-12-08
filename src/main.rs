use clap::{arg, Command};

use advent_of_code::{day1, day2, day3};

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

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("day1", sub_matches)) => day1(sub_matches),
        Some(("day2", sub_matches)) => day2(sub_matches),
        Some(("day3", sub_matches)) => day3(sub_matches),
        _ => unreachable!(),
    }
}
