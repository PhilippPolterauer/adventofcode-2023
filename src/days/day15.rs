fn hash(chars: &str) -> i64 {
    let mut hash = 0;
    for c in chars.chars() {
        if c == '\n' {
            continue;
        }
        hash += c as i64;
        hash *= 17;
        hash = hash % 256;
    }
    hash
}

pub fn part1(input: &str) -> i64 {
    let mut solution = 0;
    for split in input.split(',').into_iter() {
        solution += hash(split);
    }
    solution
}
pub fn part2(_input: &str) -> i64 {
    0
}
