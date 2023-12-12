use std::collections::HashMap;

fn count_springs(line: &[u8]) -> Vec<usize> {
    line.split(|s| s == &0)
        .map(|line| line.len())
        .filter(|c| c != &0)
        .collect()
}

fn print(line: &Vec<u8>) {
    for c in line {
        let a = match c {
            0 => '.',
            1 => '#',
            2 => '?',
            _ => panic!(),
        };
        print!("{}", a);
    }
}

// fn combinations()
fn find_solutions(input: &mut Vec<u8>, spring: Vec<usize>) -> i64 {
    let openidx: Vec<(usize, usize)> = input
        .iter()
        .enumerate()
        .filter_map(|(idx, s)| if s == &2 { Some(idx) } else { None })
        .enumerate()
        .collect();

    let cnt = openidx.len() as u32;

    let mut solution_cnt = 0;
    for n in 0..(2_i64.pow(cnt)) {
        for (i, idx) in openidx.iter() {
            let val = ((n >> i) & 1_i64) as u8;

            input[*idx] = val;
        }

        // println!();
        // // print(&spring);
        // print(&input);
        // dbg!(count_springs(&input));
        if spring == count_springs(&input) {
            solution_cnt += 1;
        }
    }
    solution_cnt
}

fn parse_input(input: &str) -> (Vec<Vec<u8>>, Vec<Vec<usize>>) {
    let mut inputs = Vec::new();
    let mut springs = Vec::new();
    for line in input.lines() {
        let tosolve: Vec<u8> = line
            .split(' ')
            .nth(0)
            .unwrap()
            .chars()
            .map(|c| match c {
                '.' => 0,
                '#' => 1,
                '?' => 2,
                _ => panic!("should not happen"),
            })
            .collect();
        inputs.push(tosolve);

        let spring: Vec<usize> = line
            .split(' ')
            .nth(1)
            .unwrap()
            .split(",")
            .map(|str| str.parse::<usize>().unwrap())
            .collect();
        springs.push(spring);
        // tosolve.chars().filter(|c| c == &'?').count();
    }
    (inputs, springs)
}

pub fn part1(input: &str) -> i64 {
    let (inputs, springs) = parse_input(input);
    let mut solution = 0;
    for (input, spring) in inputs.iter().zip(springs) {
        let mut input = input.clone();
        let solution_count = find_solutions(&mut input, spring);
        solution += solution_count;
    }

    solution
}
pub fn part2(input: &str) -> i64 {
    let mut solution = 0;
    let (inputs, springs) = parse_input(input);
    for (input, spring) in inputs.iter().zip(springs) {
        let mut input2 = input.clone();
        let mut spring2 = spring.clone();
        for _ in 0..4 {
            input2.push(2);
            for p in input {
                input2.push(*p);
            }
        }
        for _ in 0..4 {
            for p in spring.iter() {
                spring2.push(*p);
            }
        }

        let mut groups = vec![0; spring2.len()];
        let mut map = mem {
            map: HashMap::new(),
        };
        let solution_count = map.arrangment(0, -1, &mut groups, &input2, &spring2);
        solution += solution_count;
        dbg!(solution_count);
    }
    solution
}

struct mem {
    map: HashMap<(usize, i32, Vec<usize>), i64>,
}
impl mem {
    fn memarrang(
        &mut self,
        position: usize,
        groupidx: i32,
        groups: &mut Vec<usize>,
        data: &Vec<u8>,
        result: &Vec<usize>,
    ) -> i64 {
        let key = (position, groupidx, groups.to_owned());
        if let Some(val) = self.map.get(&key) {
            return *val;
        }
        let res = self.arrangment(position, groupidx, groups, data, result);
        self.map.insert(key, res);
        res
    }

    fn arrangment(
        &mut self,
        position: usize,
        groupidx: i32,
        groups: &mut Vec<usize>,
        data: &Vec<u8>,
        result: &Vec<usize>,
    ) -> i64 {
        let n = data.len();
        if position == n {
            if groups == result {
                return 1;
            } else {
                return 0;
            }
        }
        let current = data[position];
        let ingroup = groupidx >= 0;

        let gidx = groupidx as usize;

        if ingroup && (groups[gidx] > result[gidx] || gidx > groups.len()) {
            return 0;
        }

        match current {
            2 => {
                // ? mark case
                let mut groupsa = groups.clone();
                if ingroup {
                    // TODO solve with bigger step sizes in the algorithm
                    groupsa[gidx] += 1;
                    

                    return self.memarrang(position + 1, groupidx, &mut groupsa, data, result)
                        + self.memarrang(position + 1, -(groupidx + 2), groups, data, result);
                } else {
                    let groupidxa = -groupidx - 1;
                    if groupidxa as usize == groups.len() {
                        return self.memarrang(position + 1, groupidx, groups, data, result);
                    }
                    groupsa[groupidxa as usize] += 1;
                    return self.memarrang(position + 1, groupidxa, &mut groupsa, data, result)
                        + self.memarrang(position + 1, groupidx, groups, data, result);
                }
            }
            1 => {
                if ingroup {
                    groups[gidx] += 1;

                    return self.memarrang(position + 1, groupidx, groups, data, result);
                } else {
                    let groupidx = -groupidx - 1;
                    if groupidx as usize == groups.len() {
                        return 0;
                    }
                    groups[groupidx as usize] += 1;
                    return self.memarrang(position + 1, groupidx, groups, data, result);
                }
            }
            0 => {
                if ingroup {
                    // we disable the groups
                    return self.memarrang(position + 1, -(groupidx + 2), groups, data, result);
                } else {
                    return self.memarrang(position + 1, groupidx, groups, data, result);
                }
            }
            _ => panic!(""),
        }
    }
}
