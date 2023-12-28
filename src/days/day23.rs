use std::collections::{HashMap, HashSet};

use crate::util::*;

#[derive(Debug, PartialEq, Clone)]
enum MapTile {
    Path,
    Forest,
    NorthSlope,
    EastSlope,
    SouthSlope,
    WestSlope,
}
impl FromChar for MapTile {
    fn default() -> Self {
        Self::Forest
    }
    fn from_char(char: &char) -> Self {
        use MapTile::*;
        match char {
            '.' => Path,
            '#' => Forest,
            '^' => NorthSlope,
            '>' => EastSlope,
            '<' => WestSlope,
            'v' => SouthSlope,
            _ => panic!("should not happen!"),
        }
    }
}

fn state_input_valid(state: &State, input: &Direction, map: &Matrix<MapTile>) -> bool {
    use Direction::*;
    use MapTile::*;
    let elem = &map[state.idx];
    &state.last.opposite() != input
        && match elem {
            NorthSlope => input == &Up,
            EastSlope => input == &Right,
            SouthSlope => input == &Down,
            WestSlope => input == &Left,
            _ => true,
        }
}
fn state_valid(state: &State, map: &Matrix<MapTile>) -> bool {
    let elem = &map[state.idx];
    use MapTile::*;
    match elem {
        Forest => false,
        _ => true,
    }
}

impl MatrixElement for MapTile {}
#[derive(Debug, PartialEq, Eq, Hash)]
struct State {
    idx: MatrixIdx,
    last: Direction,
    cost: i64,
}

pub fn part1(input: &str) -> i64 {
    let mut costmap = HashMap::new();
    let map = Matrix::<MapTile>::from_string(input);
    let start = MatrixIdx { col: 1, row: 0 };

    let mut front = HashSet::from([State {
        idx: start,
        last: Direction::Down,
        cost: 0,
    }]);
    let goal = MatrixIdx {
        col: map.width() - 2,
        row: map.height() - 1,
    };
    while !front.is_empty() {
        let mut next_front = HashSet::new();
        for state in front.into_iter().filter(|state| state_valid(state, &map)) {
            let State { idx, cost, .. } = state;
            if let Some(old) = costmap.get(&idx) {
                if &cost > old {
                    costmap.insert(idx, cost);
                } else {
                    continue;
                }
            } else {
                costmap.insert(idx, cost);
            }
            let neighbours = map.neighbour_idzs_dir(&idx);
            for (nidx, dir) in neighbours {
                if state_input_valid(&state, &dir, &map) {
                    let next = State {
                        idx: nidx,
                        last: dir,
                        cost: cost + 1,
                    };
                    next_front.insert(next);
                }
            }
        }
        front = next_front;
    }
    costmap[&goal]
}
pub fn part2(_input: &str) -> i64 {
    0
}
