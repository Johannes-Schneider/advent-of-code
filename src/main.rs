use clap::{arg, Command};

use advent_of_code::{day1, day10, day11, day12, day2, day3, day4, day5, day6, day7, day8, day9};

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
        .subcommand(
            Command::new("day4")
                .arg(arg!(<FILE> "The input file for the challenge."))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("day5")
                .arg(arg!(<FILE> "The input file for the challenge."))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("day6")
                .arg(arg!(<FILE> "The input file for the challenge."))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("day7")
                .arg(arg!(<FILE> "The input file for the challenge."))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("day8")
                .arg(arg!(<FILE> "The input file for the challenge."))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("day9")
                .arg(arg!(<FILE> "The input file for the challenge."))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("day10")
                .arg(arg!(<FILE> "The input file for the challenge."))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("day11")
                .arg(arg!(<FILE> "The input file for the challenge."))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("day12")
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
        Some(("day4", sub_matches)) => day4(sub_matches),
        Some(("day5", sub_matches)) => day5(sub_matches),
        Some(("day6", sub_matches)) => day6(sub_matches),
        Some(("day7", sub_matches)) => day7(sub_matches),
        Some(("day8", sub_matches)) => day8(sub_matches),
        Some(("day9", sub_matches)) => day9(sub_matches),
        Some(("day10", sub_matches)) => day10(sub_matches),
        Some(("day11", sub_matches)) => day11(sub_matches),
        Some(("day12", sub_matches)) => day12(sub_matches),
        _ => unreachable!(),
    }
}
