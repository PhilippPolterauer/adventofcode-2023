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
    !matches!(elem, Forest)
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

fn next_elems(idx: &MatrixIdx, next: &MatrixIdx, map: &Matrix<MapTile>) -> Vec<MatrixIdx> {
    map.neighbour_idzs(next)
        .into_iter()
        .filter(|nidx| map[*nidx] != MapTile::Forest && nidx != idx)
        .collect()
}

fn find_path(idx: &MatrixIdx, next: &MatrixIdx, map: &Matrix<MapTile>) -> (MatrixIdx, usize) {
    let mut idx = *idx;
    let mut next = *next;
    let mut nexts = next_elems(&idx, &next, map);
    let mut cnt = 0;
    while !nexts.is_empty() && nexts.len() == 1 {
        cnt += 1;
        idx = next;
        next = nexts[0];
        nexts = next_elems(&idx, &next, map);
    }

    (next, cnt + 1)
}

pub fn part2(input: &str) -> i64 {
    // let mut costmap = HashMap::new();
    let map = Matrix::<MapTile>::from_string(input);
    let _visitmap: HashMap<MatrixIdx, HashSet<Vec<u8>>> = HashMap::new();
    let start = MatrixIdx { col: 1, row: 0 };
    let goal = MatrixIdx {
        col: map.width() - 2,
        row: map.height() - 1,
    };

    let idzs = map.findall(|e| e != &MapTile::Forest);

    let mut graph = Graph::new();
    for idx in idzs {
        let neighbours = map.neighbour_idzs_filt(&idx, |e| e != &MapTile::Forest);

        if neighbours.len() > 2 {
            graph.add_node(idx);
            for next in neighbours {
                let (goal, distance) = find_path(&idx, &next, &map);
                graph.add_node(goal);
                graph.add_edge(&idx, &goal, distance);
                graph.add_edge(&goal, &idx, distance);
            }
        }
    }
    graph.find_longest_path(&start, &goal) as i64
    // costmap[&goal]
}
