pub fn part1(input: String) -> i64 {    let mut solution = 0;
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
            
            diff += current.last().unwrap();
            current.push(diff);
            
        }
        
        solution += diff;
    }
    solution as i64
}
pub fn part2(input: String) -> i64 {    let mut solution = 0;
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
        // 
        solution += diff;
    }
    solution as i64
}
