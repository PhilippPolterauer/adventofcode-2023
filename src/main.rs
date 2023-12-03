mod day1;
mod day2;
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
        _ => (),
    }
}
