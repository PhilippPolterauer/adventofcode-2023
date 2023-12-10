use std::collections::HashSet;
fn parse_numbers(input: &str) -> HashSet<i32> {
    let iter = input.split(' ');
    let mut numbers = HashSet::new();
    for numstr in iter {
        if numstr.is_empty() {
            continue;
        }
        numbers.insert(numstr.to_string().parse::<i32>().unwrap());
    }
    numbers
}
pub fn part1(input: &str) -> i64 {
    let mut sum = 0;
    for game in input.lines() {
        let temp = game.split(':').nth(1).unwrap();
        let mut iter = temp.split('|');
        let winning = parse_numbers(iter.next().unwrap());
        let numbers = parse_numbers(iter.next().unwrap());
        let hits: Vec<_> = winning.intersection(&numbers).collect();

        if !hits.is_empty() {
            let add = i32::pow(2, (hits.len() - 1) as u32);
            sum += add;
        }
    }
    sum as i64
}
pub fn part2(input: &str) -> i64 {
    let ncards = input.lines().count();
    let mut carddeck = vec![1; ncards];
    for (cardidx, game) in input.lines().enumerate() {
        let temp = game.split(':').nth(1).unwrap();
        let mut iter = temp.split('|');
        let winning = parse_numbers(iter.next().unwrap());
        let numbers = parse_numbers(iter.next().unwrap());
        let hits: Vec<_> = winning.intersection(&numbers).collect();

        let stop = std::cmp::min(ncards - 1, cardidx + hits.len());
        let cardcnt = carddeck[cardidx];
        for card in carddeck.iter_mut().take(stop + 1).skip(cardidx + 1) {
            *card += cardcnt;
        }
    }

    let sum = carddeck.iter().sum::<i32>();
    sum as i64
}

#[cfg(test)]
mod tests {}
