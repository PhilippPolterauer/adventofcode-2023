fn parse_line(line: &str) -> u64 {
    let first = line.chars().find(|c| c.is_ascii_digit()).unwrap();
    let last = line.chars().rev().find(|c| c.is_ascii_digit()).unwrap();

    (first.to_string() + &last.to_string())
        .parse::<u64>()
        .unwrap()
}

const REPLACEMENTS: [(&str, char); 9] = [
    ("one", '1'),
    ("two", '2'),
    ("three", '3'),
    ("four", '4'),
    ("five", '5'),
    ("six", '6'),
    ("seven", '7'),
    ("eight", '8'),
    ("nine", '9'),
];

fn matches_patterns(line: &str, start: usize, patterns: (&str, char)) -> Option<char> {
    let startline = line[start..].to_owned();
    let (text, number) = patterns;
    if startline.starts_with(text) || startline.starts_with(number) {
        Some(number)
    } else {
        None
    }
}

fn convert_ascii(line: &str) -> String {
    let mut linecvt = "".to_owned();

    for start in 0..line.len() {
        for pattern in REPLACEMENTS.iter() {
            if let Some(number) = matches_patterns(line, start, *pattern) {
                linecvt += &number.to_string();
            }
        }
    }
    linecvt
}

pub fn part1(input: &str) -> i64 {
    // for each line find first and last digit
    let mut sum = 0;
    for line in input.lines() {
        let num = parse_line(line);
        // num);
        sum += num;
    }
    sum as i64
}
pub fn part2(input: &str) -> i64 {
    // for each line find first and last digit
    let mut sum = 0;

    for line in input.lines() {
        let linecvt = convert_ascii(line);
        let num = parse_line(linecvt.as_str());
        sum += num;
    }
    sum as i64
}
