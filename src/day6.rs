fn parse_line(line: &str) -> Vec<i64> {
    let mut numbers = vec![];
    let mut iter = line.split_whitespace();
    iter.next();
    for num in iter {
        numbers.push(num.to_string().parse::<i64>().unwrap());
    }
    numbers
}

fn compute_limits(time: i64, distance: i64) -> (f64, f64) {
    // function for computing the distance (d) given the race time (t) and the inputtime (x)
    // d = (t-x)*x
    // d = t*x - x^2
    // 0 = t - 2x -> x=t/2
    // given a current record D the limits are x^2 - t*x + D = 0 => xr = t/2 +- sqrt(t^2/4 -D)
    let t = time as f64;
    let d = distance as f64;
    (t / 2., (t.powi(2) / 4. - d).sqrt())
}

fn parse_races(input: String) -> Vec<(i64, i64)> {
    let times = parse_line(input.lines().nth(0).unwrap());
    let distance = parse_line(input.lines().nth(1).unwrap());
    times.into_iter().zip(distance).collect()
}

fn compute_solution_count((time, distance): (i64, i64)) -> i64 {
    let (mid, delta) = compute_limits(time, distance);
    let maxt = (mid + delta - f64::EPSILON.sqrt()).floor() as i64;
    let mint = (mid - delta + f64::EPSILON.sqrt()).ceil() as i64;
    maxt - mint + 1
}

pub fn part1(input: String) {
    let races = parse_races(input);

    let mut solution = 1;
    for race in races {
        dbg!(&race);
        let count = compute_solution_count(race);
        solution *= count;
        dbg!(count);
    }

    dbg!(solution);
}
pub fn part2(input: String) {
    let races = parse_races(input);

    let mut solution = 1;
    for race in races {
        dbg!(&race);
        let count = compute_solution_count(race);
        solution *= count;
        dbg!(count);
    }

    dbg!(solution);
}
