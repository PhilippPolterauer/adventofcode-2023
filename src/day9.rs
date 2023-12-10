pub fn part1(input: String) {
    let mut solution = 0;
    for line in input.lines() {
        let numbers: Vec<i64> = line
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        dbg!(&numbers);
        let mut number_stack = vec![numbers.clone()];
        let mut current = numbers;
        let mut done = false;
        while !done {
            current = current.windows(2).map(|a| a[1] - a[0]).collect();
            number_stack.push(current.clone());

            done = current.iter().all(|n| *n == 0);
        }
        let mut diff = 0;
        for current in number_stack.iter_mut().rev() {
            dbg!(&current);
            diff += current.last().unwrap();
            current.push(diff);
            dbg!(&current);
        }
        dbg!(diff);
        solution += diff;
    }
    dbg!(solution);
}
pub fn part2(input: String) {
    let mut solution = 0;
    for line in input.lines() {
        let numbers: Vec<i64> = line
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        let mut number_stack = vec![numbers.clone()];
        let mut current = numbers;
        let mut done = false;
        while !done {
            current = current.windows(2).map(|a| a[1] - a[0]).collect();
            number_stack.push(current.clone());

            done = current.iter().all(|n| *n == 0);
        }
        let mut diff = 0;
        for current in number_stack.iter_mut().rev() {
            diff = current.first().unwrap() - diff;
            current.insert(0, diff);
        }
        // dbg!(diff);
        solution += diff;
    }
    dbg!(solution);
}
