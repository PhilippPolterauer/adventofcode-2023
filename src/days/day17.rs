use std::iter;
use std::{collections::HashMap, ops::Index};
#[derive(PartialEq, Hash, Eq, Clone, Debug)]
struct Board {
    data: Vec<u8>,
    width: usize,
    cost_map: Vec<u64>,
}
impl Index<&usize> for Board {
    type Output = u8;

    fn index(&self, idx: &usize) -> &u8 {
        &self.data[*idx]
    }
}

impl Board {
    fn from_string(input: &str) -> Self {
        let mut data = Vec::new();
        let mut width = 0;
        for line in input.lines() {
            width = line.len();
            for c in line.chars() {
                data.push(c as u8 - '0' as u8);
            }
        }
        let cost_map = vec![u64::MAX; data.len()];
        Self {
            data,
            width,
            cost_map,
        }
    }
    fn height(&self) -> usize {
        self.data.len() / self.width
    }
    fn prev(&self, idx: &usize, dir: &Direction) -> usize {
        let m = self.width;
        match dir {
            Direction::Up => idx + m,
            Direction::Down => idx - m,
            Direction::Left => idx + 1,
            Direction::Right => idx - 1,
        }
    }
    fn next(&self, state: &State, direction: &Direction) -> Option<State> {
        let State {
            mut idx,
            direction: last,
            mut repetition,
        } = state;
        let m = self.width;

        if self.is_possible(state, direction) {
            if repetition == 255 {
                repetition = 0;
            } else {
                repetition = if last == direction { repetition + 1 } else { 0 };
            }
            match direction {
                Direction::Up => idx -= m,
                Direction::Down => idx += m,
                Direction::Left => idx -= 1,
                Direction::Right => idx += 1,
            }

            Some(State {
                idx,
                direction: *direction,
                repetition,
            })
        } else {
            None
        }
    }
    fn forbidden_dir(&self, idx: &usize, last: &Direction) -> Vec<Direction> {
        let mut forbidden = Vec::new();
        let n = self.height() - 1;
        let m = self.width - 1;
        let col = idx % self.width;
        let row = idx / self.width;
        match last {
            Direction::Up => forbidden.push(Direction::Down),
            Direction::Down => forbidden.push(Direction::Up),
            Direction::Left => forbidden.push(Direction::Right),
            Direction::Right => forbidden.push(Direction::Left),
        };
        if col == 0 {
            forbidden.push(Direction::Left);
        }
        if col == m {
            forbidden.push(Direction::Right);
        }
        if row == 0 {
            forbidden.push(Direction::Up);
        }
        if row == n {
            forbidden.push(Direction::Down);
        }

        forbidden
    }
    fn is_possible(&self, state: &State, dir: &Direction) -> bool {
        let last = state.direction;
        let num = state.repetition;

        if dir == &last && num == 2 {
            return false;
        }

        if self.forbidden_dir(&state.idx, &last).contains(dir) {
            return false;
        } else {
            return true;
        }
    }
}
#[derive(PartialEq, Clone, Copy, Hash, Eq, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
impl From<usize> for Direction {
    fn from(value: usize) -> Self {
        match value {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            3 => Direction::Left,
            _ => panic!("should not happen!"),
        }
    }
}
const ALL_DIR: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];
#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
struct State {
    idx: usize,
    direction: Direction,
    repetition: u8,
}

pub fn part1(input: &str) -> i64 {
    let board = Board::from_string(input);
    // for each x in xg we compute the min as
    let start = State {
        idx: 0,
        direction: Direction::Up,
        repetition: 255,
    };
    let goal = board.data.len() - 1;
    let mut front = vec![start];
    let mut cost = vec![[[i64::MAX; 3]; 4]; board.data.len()];
    while !front.is_empty() {
        let mut next_front = Vec::new();
        for current in front.iter() {
            for dir in ALL_DIR {
                if let Some(next) = board.next(current, &dir) {
                    let accum_cost = if current.repetition == 255 {
                        0
                    } else {
                        cost[current.idx][current.direction as usize][current.repetition as usize]
                    };
                    let next_cost = accum_cost + board[&next.idx] as i64;
                    let current_cost =
                        cost[next.idx][next.direction as usize][next.repetition as usize];
                    if next_cost < current_cost {
                        cost[next.idx][next.direction as usize][next.repetition as usize] =
                            next_cost;
                        next_front.push(next);
                    }
                }
            }
        }
        front = next_front;
    }
    // dbg!(&cost);
    // find minimum in last index

    let minimum = cost[goal]
        .iter()
        .enumerate()
        .flat_map(|(direction, slice)| iter::repeat(direction).zip(slice.iter().enumerate()))
        .min_by_key(|(_, (_, cost))| *cost).unwrap();
    //

    // let mut current = goal;
    // // backtrack and print index in map
    // while current != 0 {
    //     let minimum = cost[current]
    //         .iter()
    //         .enumerate()
    //         .flat_map(|(direction, slice)| iter::repeat(direction).zip(slice.iter().enumerate()))
    //         .min_by_key(|(_, (_, cost))| *cost).unwrap();
    //     let dir: Direction = minimum.0.into();
    //     dbg!(&current, &dir, board[&current], minimum.1.1);
    //     current = board.prev(&current, &dir);
    // }
    *minimum.1.1
}
pub fn part2(_input: &str) -> i64 {
    0
}
