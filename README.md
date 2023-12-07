# Adventofcode 2023

## Useage

the projects main entry is a cli program that can run all the days challenges.

```console
philipp@Philipp:~/rust/adventofcode$ cargo run -- --help
    Finished dev [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/adventofcode --help`
Solving adventofcode Challenges

Usage: adventofcode [OPTIONS] <DAY> <PART>

Arguments:
  <DAY>   
  <PART>  

Options:
  -t, --test  use short test dataset
  -h, --help  Print help
```

so if you want to run they 1 part 2 with the test data set run

```console
philipp@Philipp:~/rust/adventofcode$ cargo run 1 2 -t
    Finished dev [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/adventofcode 1 2 -t`
day: 1, part: 2
loading data from 'data/day1/test_input2.txt'
...

[src/day1.rs:61] line = "7pqrstsixteen"
[src/day1.rs:62] &linecvt = "76"
[src/day1.rs:63] &num = 76
[src/day1.rs:66] sum = 281
```

## About

Author: Philipp Polterauer
