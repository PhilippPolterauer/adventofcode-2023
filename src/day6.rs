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

pub fn part1(input: String) {
    let times = parse_line(input.lines().nth(0).unwrap());
    let distance = parse_line(input.lines().nth(1).unwrap());
    let races = times.iter().zip(distance);

    let mut solution = 1;
    for (time, distance) in races {
        dbg!(&time, &distance);
        let (mid, delta) = compute_limits(*time, distance);
        let maxt = (mid + delta - f64::EPSILON.sqrt()).floor() as i64;
        let mint = (mid - delta + f64::EPSILON.sqrt()).ceil() as i64;
        let count = maxt - mint + 1;
        dbg!(mint, maxt, count);
        solution *= count;
    }

    

    dbg!(solution);
}
pub fn part2(input: String) {}
