use clap::{arg, Parser};

/// Solving adventofcode Challenges 
#[derive(Parser)]
struct Cli {
    day: u8,
    part: u8,
    #[arg(short = 't', default_value = "false")]
    runtest: bool,
}
mod day1;

fn main() {
    let args = Cli::parse();

    println!("day: {:?}, part: {:?}", args.day, args.part);
    let Cli { day, part, runtest } = args;
    if day == 1 {
        if part == 1 {
            day1::part1(runtest)
        }
        if part == 2 {
            day1::part2(runtest)
        }
    }
}
