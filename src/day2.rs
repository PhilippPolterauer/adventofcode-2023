use crate::util;

const MAXRED: i32 = 12;
const MAXGREEN: i32 = 13;
const MAXBLUE: i32 = 14;

fn maxcubes<'a, I>(shows: I) -> (i32, i32, i32)
where
    I: Iterator<Item = &'a str>,
{
    let mut reds = vec![];
    let mut greens = vec![];
    let mut blues = vec![];
    for show in shows {
        for col in show.split(",") {
            let mut split = col.split(" ");
            let cnt = split.nth(1).unwrap().parse::<i32>().unwrap();
            let color = split.next().unwrap();
            match color {
                "red" => reds.push(cnt),
                "green" => greens.push(cnt),
                "blue" => blues.push(cnt),
                _ => (),
            }
        }
    }
    (
        reds.iter().max().unwrap().to_owned(),
        greens.iter().max().unwrap().to_owned(),
        blues.iter().max().unwrap().to_owned(),
    )
}
pub fn part1(input: String) {
    let mut solution = 0;
    for game in input.lines() {
        let id = game
            .split(":")
            .next()
            .unwrap()
            .split(" ")
            .nth(1)
            .unwrap()
            .parse::<u64>()
            .unwrap();

        let shows = game.split(":").nth(1).unwrap().split(";");

        let (maxred, maxgreen, maxblue) = maxcubes(shows);
        let valid = maxred <= MAXRED && maxgreen <= MAXGREEN && maxblue <= MAXBLUE;
        if valid {
            solution += id;
        }
    }
    dbg!(solution);
}

pub fn part2(input: String) {
    let mut solution = 0;
    for game in input.lines() {
        let id = game
            .split(":")
            .next()
            .unwrap()
            .split(" ")
            .nth(1)
            .unwrap()
            .parse::<u64>()
            .unwrap();

        let shows = game.split(":").nth(1).unwrap().split(";");

        let (maxred, maxgreen, maxblue) = maxcubes(shows);

        solution += maxred * maxgreen * maxblue;
    }
    dbg!(solution);
}
