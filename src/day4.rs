use std::collections::HashSet;
fn parse_numbers(input: String) -> HashSet<i32> {
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
pub fn part1(input: String) {
    let mut sum = 0;
    for game in input.lines() {
        let temp = game.split(':').nth(1).unwrap();
        let mut iter = temp.split('|');
        let winning = parse_numbers(iter.next().unwrap().to_string());
        let numbers = parse_numbers(iter.next().unwrap().to_string());
        let hits: Vec<_> = winning.intersection(&numbers).collect();
        dbg!(&hits);
        if !hits.is_empty() {
            let add = i32::pow(2, (hits.len() - 1) as u32);
            sum += add;
            dbg!(add);
        }
    }
    dbg!(sum);
}
pub fn part2(input: String) {
    let ncards = input.lines().count();
    let mut carddeck = vec![1; ncards];
    for (cardidx, game) in input.lines().enumerate() {
        let temp = game.split(':').nth(1).unwrap();
        let mut iter = temp.split('|');
        let winning = parse_numbers(iter.next().unwrap().to_string());
        let numbers = parse_numbers(iter.next().unwrap().to_string());
        let hits: Vec<_> = winning.intersection(&numbers).collect();
        dbg!(&hits);

        let stop = std::cmp::min(ncards - 1, cardidx + hits.len());
        let cardcnt = carddeck[cardidx];
        for idx in cardidx + 1..=stop {
            carddeck[idx] += cardcnt
        }
    }
    dbg!(&carddeck);
    let sum = carddeck.iter().sum::<i32>();
    dbg!(sum);
}

#[cfg(test)]
mod tests {}
