mod days;
mod util;
use std::time::Instant;

use clap::{arg, Parser};

use crate::days::dispatch_function;

/// Solving adventofcode challenges
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
    #[arg(
        short = 'd',
        long = "data",
        default_value = "data",
        help = "path to the input data folder"
    )]
    data: String,
    #[arg(
        short = 'p',
        long = "profile",
        default_value = "false",
        help = "repeat runs x1000 for profiling"
    )]
    profile: bool,
    #[arg(
        short = 'n',
        long = "numruns",
        default_value = "1000",
        help = "how often to repat the function call for profiling"
    )]
    numruns: i64,
}

fn main() {
    let args = Cli::parse();

    println!("day: {:?}, part: {:?}", args.day, args.part);
    let Cli {
        day,
        part,
        runtest,
        data,
        profile,
        numruns,
    } = args;
    let input = util::load_file(day, part, runtest, &data);

    let function = dispatch_function(day, part);

    if profile {
        for _ in 0..numruns {
            let sol = function(&input);
            dbg!(sol);
        }
    }
    let t0 = Instant::now();
    let solution = function(&input);
    println!("Duration: {} us", (Instant::now() - t0).as_micros());
    println!("Solution: {}", solution);
}
