use clap::{arg, ArgMatches, Command};

mod day1;

fn cli() -> Command {
    Command::new("aoc")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("day1")
                .arg(arg!(<FILE> "The input file for the challenge."))
                .arg_required_else_help(true),
        )
}

fn day1(sub_matches: &ArgMatches) {
    let file = sub_matches.get_one::<String>("FILE").expect("required arg");
    let maybe_result = day1::day1(file);
    println!("Result of Day1 challenge: {}", maybe_result.unwrap());
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("day1", sub_matches)) => {
            day1(sub_matches);
        }
        _ => unreachable!(),
    }
}
