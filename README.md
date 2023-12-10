# Adventofcode 2023

[![CI](https://github.com/PhilippPolterauer/adventofcode-2023/actions/workflows/ci.yaml/badge.svg)](https://github.com/PhilippPolterauer/adventofcode-2023/actions/workflows/ci.yaml)

## Useage

the projects main entry is a cli program that can run all the days challenges.

```console
$ adventofcode --help
Solving adventofcode Challenges

Usage: adventofcode [OPTIONS] <DAY> <PART>

Arguments:
  <DAY>   
  <PART>  

Options:
  -t, --test         use short test dataset
  -d, --data <DATA>  path to the input data folder [default: data]
  -p, --profile      repeat runs x1000 for profiling
  -h, --help         Print help

```

so if you want to run they 1 part 2 with the test data set run

```console
$ adventofcode 1 1 -t
day: 1, part: 1
loading data from 'data/day1/test_input1.txt'
Solution: 142

```

## Solutions

all solutions are found below

### Day 1

#### Part 1

```console
$ adventofcode 1 1
day: 1, part: 1
loading data from 'data/day1/input1.txt'
Solution: 55447

```

#### Part 2

```console
$ adventofcode 1 2
day: 1, part: 2
loading data from 'data/day1/input2.txt'
Solution: 54706

```

### Day 2

#### Part 1

```console
$ adventofcode 2 1
day: 2, part: 1
loading data from 'data/day2/input1.txt'
Solution: 2348

```

#### Part 2

```console
$ adventofcode 2 2
day: 2, part: 2
loading data from 'data/day2/input2.txt'
Solution: 76008

```

### Day 3

#### Part 1

```console
$ adventofcode 3 1
day: 3, part: 1
loading data from 'data/day3/input1.txt'
Solution: 528799

```

#### Part 2

```console
$ adventofcode 3 2
day: 3, part: 2
loading data from 'data/day3/input2.txt'
Solution: 84907174

```

### Day 4

#### Part 1

```console
$ adventofcode 4 1
day: 4, part: 1
loading data from 'data/day4/input1.txt'
Solution: 20407

```

#### Part 2

```console
$ adventofcode 4 2
day: 4, part: 2
loading data from 'data/day4/input2.txt'
Solution: 23806951

```

### Day 5

#### Part 1

```console
$ adventofcode 5 1
day: 5, part: 1
loading data from 'data/day5/input1.txt'
Solution: 289863851

```

#### Part 2

```console
$ adventofcode 5 2
day: 5, part: 2
loading data from 'data/day5/input2.txt'
Solution: 60568880

```

### Day 6

#### Part 1

```console
$ adventofcode 6 1
day: 6, part: 1
loading data from 'data/day6/input1.txt'
Solution: 4811940

```

#### Part 2

```console
$ adventofcode 6 2
day: 6, part: 2
loading data from 'data/day6/input2.txt'
Solution: 30077773

```

### Day 7

#### Part 1

```console
$ adventofcode 7 1
day: 7, part: 1
loading data from 'data/day7/input1.txt'
Solution: 248179786

```

#### Part 2

```console
$ adventofcode 7 2
day: 7, part: 2
loading data from 'data/day7/input2.txt'
Solution: 247885995

```

### Day 8

#### Part 1

```console
$ adventofcode 8 1
day: 8, part: 1
loading data from 'data/day8/input1.txt'
Solution: 13939

```

#### Part 2

```console
$ adventofcode 8 2
day: 8, part: 2
loading data from 'data/day8/input2.txt'
Solution: 8906539031197

```

### Day 9

#### Part 1

```console
$ adventofcode 9 1
day: 9, part: 1
loading data from 'data/day9/input1.txt'
Solution: 1684566095

```

#### Part 2

```console
$ adventofcode 9 2
day: 9, part: 2
loading data from 'data/day9/input2.txt'
Solution: 1136

```

### Day 10

#### Part 1

```console
$ adventofcode 10 1
day: 10, part: 1
loading data from 'data/day10/input1.txt'
Solution: 7066

```

#### Part 2

```console
$ adventofcode 10 2
day: 10, part: 2
loading data from 'data/day10/input2.txt'
Solution: 401

```

<!-- 
### Day 11

#### Part 1

```console
$ adventofcode 11 1
? 101
day: 11, part: 1
loading data from 'data/day11/input1.txt'
thread 'main' panicked at src/util.rs:6:35:
called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

```

#### Part 2

```console
$ adventofcode 11 2
? 101
day: 11, part: 2
loading data from 'data/day11/input2.txt'
thread 'main' panicked at src/util.rs:6:35:
called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

```

### Day 12

#### Part 1

```console
$ adventofcode 12 1
? 101
day: 12, part: 1
loading data from 'data/day12/input1.txt'
thread 'main' panicked at src/util.rs:6:35:
called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

```

#### Part 2

```console
$ adventofcode 12 2
? 101
day: 12, part: 2
loading data from 'data/day12/input2.txt'
thread 'main' panicked at src/util.rs:6:35:
called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

``` -->

## About

Author: Philipp Polterauer
