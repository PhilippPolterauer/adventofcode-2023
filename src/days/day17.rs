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
    fn next(
        &self,
        state: &State,
        direction: &Direction,
        possfun: fn(&Self, &State, &Direction) -> bool,
    ) -> State {
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
    fn prev(&self, state: &State, direction: &Direction) -> State {
        let State {
            mut idx,
            direction: last,
            mut repetition,
        } = state;
        let m = self.width;

        repetition = if last == direction { repetition - 1 } else { 9 };

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
    fn forbidden_dir(&self, state: &State) -> Vec<Direction> {
        let State { idx, direction, .. } = state;
        let mut forbidden = Vec::new();
        let n = self.height() - 1;
        let m = self.width - 1;
        let col = idx % self.width;
        let row = idx / self.width;

        match direction {
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

        if self.forbidden_dir(&state).contains(dir) {
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
fn solve(
    board: &Board,
    front: Vec<State>,
    cost: &mut Vec<Vec<Vec<i64>>>,
    possfun: fn(&Board, &State, &Direction) -> bool,
) {
    let mut front = front;
    while !front.is_empty() {
        let mut next_front = Vec::new();
        for current in front.iter() {
            for dir in allowed_dirs(current, board) {
                let next = board.next(current, &dir, possfun);
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

fn allowed_dirs(state: &State, board: &Board) -> HashSet<Direction> {
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

    if repetition == &0 {
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

fn solve_backward(board: &Board, front: Vec<State>, cost: &mut Vec<Vec<Vec<i64>>>) {
    let mut front = front;
    while !front.is_empty() {
        let mut prev_front = Vec::new();
        for current in front {
            for dir in allowed_dirs(&current, board) {
                let prev = board.prev(&current, &dir);
                let prev_cost = cost[current.idx][current.direction as usize]
                    [current.repetition as usize]
                    + board[&current.idx] as i64;
                let current_cost =
                    cost[prev.idx][prev.direction as usize][prev.repetition as usize];
                if prev_cost < current_cost {
                    cost[prev.idx][prev.direction as usize][prev.repetition as usize] = prev_cost;
                    prev_front.push(prev);
                }
            }
        }
        front = prev_front;
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
    solve(&board, front, &mut cost, Board::is_possible);
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
    let start = 0;
    let goal = State {
        idx: board.data.len() - 1,
        direction: Direction::Up,
        repetition: 9,
    };
    let goal2 = State {
        idx: board.data.len() - 1,
        direction: Direction::Left,
        repetition: 9,
    };
    let front = vec![goal,goal2];
    let mut cost = vec![vec![vec![i64::MAX; 10]; 4]; board.data.len()];
    cost[goal.idx][0][9]=0;
    cost[goal.idx][3][9]=0;
    solve_backward(&board, front, &mut cost);
    // find minimum in last index
    let minimum = cost[start]
        .iter()
        .enumerate()
        .flat_map(|(direction, slice)| iter::repeat(direction).zip(slice.iter().enumerate()))
        .min_by_key(|(_, (_, cost))| *cost)
        .unwrap();

    *minimum.1 .1
}
