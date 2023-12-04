use std::collections::HashSet;
fn parse_numbers(input: String) -> HashSet<i32> {
    let iter = input.split(" ");
    let mut numbers = HashSet::new();
    for numstr in iter {
        if numstr.is_empty() {
            continue;
        }
        numbers.insert(numstr.to_string().parse::<i32>().unwrap());
    }
    numbers
}
pub fn part1(input: String) {
    let mut sum = 0;
    for game in input.lines() {
        let temp = game.split(":").nth(1).unwrap();
        let mut iter = temp.split("|");
        let winning = parse_numbers(iter.next().unwrap().to_string());
        let numbers = parse_numbers(iter.next().unwrap().to_string());
        let hits: Vec<_> = winning.intersection(&numbers).collect();
        dbg!(&hits);
        if hits.len() > 0 {
            let add = i32::pow(2, (hits.len() - 1) as u32);
            sum += add;
            dbg!(add);
        }
    }
    dbg!(sum);
}
pub fn part2(input: String) {}

#[cfg(test)]
mod tests {}
