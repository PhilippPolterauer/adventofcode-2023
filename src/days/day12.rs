use std::{collections::HashMap, usize};

fn parse_input(input: &str) -> (Vec<Vec<u8>>, Vec<Vec<usize>>) {
    let mut inputs = Vec::new();
    let mut springs = Vec::new();
    for line in input.lines() {
        let tosolve: Vec<u8> = line
            .split(' ')
            .next()
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
            .split(',')
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
        let input = input.clone();
        let mut groups = vec![0; spring.len()];
        let mut map = Memo {
            map: HashMap::new(),
        };
        let solution_count = map.arrangment(0, 0, false, &mut groups, &input, &spring);
        solution += solution_count;
    }

    solution
}
pub fn part2(input: &str) -> i64 {
    let mut solution = 0;
    let (inputs, springs) = parse_input(input);
    for (input, spring) in inputs.iter().zip(springs) {
        let mut input_tot = input.clone();
        let mut input = input.clone();
        input.insert(0, 2);
        let mut spring_tot = spring.clone();

        for _ in 0..4 {
            for p in input.clone() {
                input_tot.push(p);
            }
        }
        for _ in 0..4 {
            for p in spring.iter() {
                spring_tot.push(*p);
            }
        }

        let mut groups = vec![0; spring_tot.len()];
        let mut map = Memo {
            map: HashMap::new(),
        };
        let solution_count = map.arrangment(0, 0, false, &mut groups, &input_tot, &spring_tot);
        solution += solution_count;
    }
    solution
}

fn could_be_spring(c: &u8) -> bool {
    match c {
        2 | 1 => true,
        0 => false,
        _ => panic!(),
    }
}
fn could_be_dry(c: &u8) -> bool {
    match c {
        1 => false,
        0 | 2 => true,
        _ => panic!(),
    }
}

fn remaining_springs(groups: &Vec<usize>, result: &[usize]) -> i64 {
    result
        .iter()
        .zip(groups)
        .map(|(r, g)| *r as i64 - *g as i64)
        .sum()
}

struct Memo {
    map: HashMap<(usize, usize, bool, Vec<usize>), i64>,
}
impl Memo {
    fn memarrang(
        &mut self,
        position: usize,
        gidx: usize,
        ingroup: bool,
        groups: &mut Vec<usize>,
        data: &Vec<u8>,
        result: &Vec<usize>,
    ) -> i64 {
        let key = (position, gidx, ingroup, groups.to_owned());
        if let Some(val) = self.map.get(&key) {
            return *val;
        }
        let res = self.arrangment(position, gidx, ingroup, groups, data, result);
        self.map.insert(key, res);
        res
    }

    fn arrangment(
        &mut self,
        position: usize,
        gidx: usize,
        ingroup: bool,
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
        let ngroups = result.len();

        if ingroup && (groups[gidx] > result[gidx] || gidx > groups.len()) {
            return 0;
        }
        let mindots_remaining = ngroups as i64 - gidx as i64 - 1;
        let a = remaining_springs(groups, result);
        if ((n - position) as i64) < (a + mindots_remaining) {
            // we need atleast reamining springs + 1 if we are not in a group
            return 0;
        }

        if ingroup {
            let cur_group_end = position + (result[gidx] - groups[gidx]);

            // TODO solve with bigger step sizes in the algorithm
            if !data[position..cur_group_end].iter().all(could_be_spring) {
                return 0;
            }
            // we only get here if the group can be satisfied
            if gidx == ngroups - 1 {
                // last group case
                if cur_group_end == n {
                    // this is the case if the current group runs until the end
                    return 1;
                }
                // we only get here when curgroup_end < n, then all remaining needs to be .
                if data[cur_group_end..].iter().all(could_be_dry) {
                    1
                } else {
                    0
                }
            } else {
                groups[gidx] = result[gidx];
                //if not the last group we make sure the point after the current group is a dot
                if !could_be_dry(&data[cur_group_end]) {
                    return 0;
                }
                let mut pos = cur_group_end + 1;
                while pos < n && data[pos] == 0 {
                    // we run until we find a # or ?
                    pos += 1;
                }
                if pos == n {
                    // we ran out but where not in the last group
                    return 0;
                }
                // here we are sure that some chars are remaining and that the current char is not 0

                // value for case = 1
                let mut groups_cpy = groups.clone();
                groups_cpy[gidx + 1] += 1;

                let val = self.memarrang(pos + 1, gidx + 1, ingroup, &mut groups_cpy, data, result);
                if data[pos] == 1 {
                    return val;
                }
                // in case of 2 we need to also compute the val for the . case
                val + self.memarrang(pos + 1, gidx + 1, false, groups, data, result)
            }
        } else {
            let ngidx = gidx;
            if ngidx == ngroups {
                // we are past the last group // the only allowed solutions is all . until end
                if data[position..].iter().all(could_be_dry) {
                    return 1;
                } else {
                    return 0;
                }
            }
            // if we get here a new group can be started
            // not in a group we proceed until a # or ? potential group
            let mut pos = position;
            while pos < n && data[pos] == 0 {
                // we run until we find a # or ?
                pos += 1;
            }
            if (n as i64 - pos as i64) < remaining_springs(groups, result) + mindots_remaining {
                return 0;
            }
            // in theory can be a feasible solution
            let mut groups_cpy = groups.clone();
            groups_cpy[ngidx] += 1;
            let val = self.memarrang(pos + 1, ngidx, true, &mut groups_cpy, data, result);
            if data[pos] == 1 {
                return val;
            }
            // if we get here its a 2
            val + self.memarrang(pos + 1, gidx, ingroup, groups, data, result)
        }
    }
}
