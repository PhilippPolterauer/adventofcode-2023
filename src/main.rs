mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod util;

use clap::{arg, Parser};

/// Solving adventofcode Challenges
#[derive(Parser)]
struct Cli {
    day: i32,
    part: i32,
    // use short test dataset
    #[arg(
        short = 't',
        long = "test",
        default_value = "false",
        help = "use short test dataset"
    )]
    runtest: bool,
}

fn main() {
    let args = Cli::parse();

    println!("day: {:?}, part: {:?}", args.day, args.part);
    let Cli { day, part, runtest } = args;
    let input = util::load_file(day, part, runtest);
    match (day, part) {
        (1, 1) => day1::part1(input),
        (1, 2) => day1::part2(input),
        (2, 1) => day2::part1(input),
        (2, 2) => day2::part2(input),
        (3, 1) => day3::part1(input),
        (3, 2) => day3::part2(input),
        (4, 1) => day4::part1(input),
        (4, 2) => day4::part2(input),
        (5, 1) => day5::part1(input),
        (5, 2) => day5::part2(input),
        (6, 1) => day6::part1(input),
        (6, 2) => day6::part2(input),
        _ => (),
    }
}
