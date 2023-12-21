use std::collections::HashSet;
use std::iter;
use std::ops::Index;
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
                data.push(c as u8 - b'0');
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
    fn next(&self, state: &State, direction: &Direction) -> State {
        let State {
            mut idx,
            direction: last,
            mut repetition,
        } = state;
        let m = self.width;

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

        State {
            idx,
            direction: *direction,
            repetition,
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
fn solve(board: &Board, front: Vec<State>, cost: &mut [Vec<Vec<i64>>]) {
    let mut front = front;
    while !front.is_empty() {
        let mut next_front = Vec::new();
        for current in front.iter() {
            for dir in allowed_dirs1(current, board) {
                let next = board.next(current, &dir);
                let accum_cost = if current.repetition == 255 {
                    0
                } else {
                    cost[current.idx][current.direction as usize][current.repetition as usize]
                };
                let next_cost = accum_cost + board[&next.idx] as i64;
                let current_cost =
                    cost[next.idx][next.direction as usize][next.repetition as usize];
                if next_cost < current_cost {
                    cost[next.idx][next.direction as usize][next.repetition as usize] = next_cost;
                    next_front.push(next);
                }
            }
        }
        front = next_front;
    }
}
fn allowed_dirs1(state: &State, board: &Board) -> HashSet<Direction> {
    let State {
        idx,
        direction,
        repetition,
    } = state;
    let mut forbidden = HashSet::new();
    let n = board.height() - 1;
    let m = board.width - 1;
    let col = idx % board.width;
    let row = idx / board.width;

    match direction {
        Direction::Up => forbidden.insert(Direction::Down),
        Direction::Down => forbidden.insert(Direction::Up),
        Direction::Left => forbidden.insert(Direction::Right),
        Direction::Right => forbidden.insert(Direction::Left),
    };
    if col == 0 {
        forbidden.insert(Direction::Left);
    }
    if col == m {
        forbidden.insert(Direction::Right);
    }
    if row == 0 {
        forbidden.insert(Direction::Up);
    }
    if row == n {
        forbidden.insert(Direction::Down);
    }

    if repetition == &2 {
        //
        forbidden.insert(*direction);
    }

    let allowed: HashSet<Direction> = ALL_DIR.into_iter().collect();

    allowed.difference(&forbidden).copied().collect()
}
fn allowed_dirs2(state: &State, board: &Board) -> HashSet<Direction> {
    let State {
        idx,
        direction,
        repetition,
    } = state;
    let mut forbidden = HashSet::new();
    let n = board.height() - 1;
    let m = board.width - 1;
    let col = idx % board.width;
    let row = idx / board.width;

    match direction {
        Direction::Up => forbidden.insert(Direction::Down),
        Direction::Down => forbidden.insert(Direction::Up),
        Direction::Left => forbidden.insert(Direction::Right),
        Direction::Right => forbidden.insert(Direction::Left),
    };
    if col == 0 {
        forbidden.insert(Direction::Left);
    }
    if col == m {
        forbidden.insert(Direction::Right);
    }
    if row == 0 {
        forbidden.insert(Direction::Up);
    }
    if row == n {
        forbidden.insert(Direction::Down);
    }

    if repetition == &9 {
        //
        forbidden.insert(*direction);
    }

    let allowed: HashSet<Direction> = if repetition < &3 {
        // we need to keep moving atleast 4 wide
        [*direction].into_iter().collect()
    } else {
        ALL_DIR.into_iter().collect()
    };

    allowed.difference(&forbidden).copied().collect()
}

fn solve2(board: &Board, front: Vec<State>, cost: &mut [Vec<Vec<i64>>]) {
    let goal = board.data.len() - 1;
    let mut front = front;
    while !front.is_empty() {
        let mut next_front = Vec::new();
        for current in front.iter() {
            for dir in allowed_dirs2(current, board) {
                let next = board.next(current, &dir);
                if next.idx == goal && next.repetition < 3 {
                    continue;
                }
                let accum_cost = if current.repetition == 255 {
                    0
                } else {
                    cost[current.idx][current.direction as usize][current.repetition as usize]
                };
                let next_cost = accum_cost + board[&next.idx] as i64;
                let current_cost =
                    cost[next.idx][next.direction as usize][next.repetition as usize];
                if next_cost < current_cost {
                    cost[next.idx][next.direction as usize][next.repetition as usize] = next_cost;
                    next_front.push(next);
                }
            }
        }
        front = next_front;
    }
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
    let front = vec![start];
    let mut cost = vec![vec![vec![i64::MAX; 3]; 4]; board.data.len()];
    solve(&board, front, &mut cost);
    // dbg!(&cost);
    // find minimum in last index

    let minimum = cost[goal]
        .iter()
        .enumerate()
        .flat_map(|(direction, slice)| iter::repeat(direction).zip(slice.iter().enumerate()))
        .min_by_key(|(_, (_, cost))| *cost)
        .unwrap();
    //

    *minimum.1 .1
}
pub fn part2(input: &str) -> i64 {
    let board = Board::from_string(input);
    // for each x in xg we compute the min as
    let start = State {
        idx: 0,
        direction: Direction::Right,
        repetition: 255,
    };
    let goal = board.data.len() - 1;
    let front = vec![start];
    let mut cost = vec![vec![vec![i64::MAX; 10]; 4]; board.data.len()];
    solve2(&board, front, &mut cost);
    // find minimum in last index
    let minimum = cost[goal]
        .iter()
        .enumerate()
        .flat_map(|(direction, slice)| iter::repeat(direction).zip(slice.iter().enumerate()))
        .min_by_key(|(_, (_, cost))| *cost)
        .unwrap();

    *minimum.1 .1
}
