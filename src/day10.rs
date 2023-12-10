use std::{collections::HashSet, fmt::Debug, ops::Index};

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
enum PipeKind {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    G,
    S,
}

#[derive(Debug, Clone, Copy)]
struct Pipe {
    idx: Idx,
    kind: PipeKind,
}

#[derive(Debug, PartialEq, Clone, Copy, PartialOrd, Eq, Ord)]
enum Direction {
    North,
    East,
    South,
    West,
}

const ALL_DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

impl Direction {
    fn oposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
    fn right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }
}

impl PipeKind {
    fn from_char(char: char) -> Self {
        match char {
            '|' => Self::NS,
            '-' => Self::EW,
            'L' => Self::NE,
            'J' => Self::NW,
            '7' => Self::SW,
            'F' => Self::SE,
            '.' => Self::G,
            'S' => Self::S,
            _ => panic!("should not get here!"),
        }
    }
    fn to_char(self) -> char {
        match self {
            Self::NS => '|',
            Self::EW => '-',
            Self::NE => 'L',
            Self::NW => 'J',
            Self::SW => '7',
            Self::SE => 'F',
            Self::G => '.',
            Self::S => 'S',
        }
    }
    fn directions(&self) -> Vec<Direction> {
        match self {
            PipeKind::NS => vec![Direction::North, Direction::South],
            PipeKind::EW => vec![Direction::East, Direction::West],
            PipeKind::NE => vec![Direction::North, Direction::East],
            PipeKind::NW => vec![Direction::North, Direction::West],
            PipeKind::SW => vec![Direction::South, Direction::West],
            PipeKind::SE => vec![Direction::South, Direction::East],
            PipeKind::G => vec![],
            PipeKind::S => vec![
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ],
        }
    }
    fn from_directions(directions: &[Direction; 2]) -> Self {
        let mut directions = *directions;
        directions.sort();
        match directions {
            [Direction::North, Direction::South] => PipeKind::NS,
            [Direction::East, Direction::West] => PipeKind::EW,
            [Direction::North, Direction::East] => PipeKind::NE,
            [Direction::North, Direction::West] => PipeKind::NW,
            [Direction::South, Direction::West] => PipeKind::SW,
            [Direction::East, Direction::South] => PipeKind::SE,
            _ => panic!(),
        }
    }
}

type Idx = (usize, usize);
struct Map {
    data: Vec<Vec<PipeKind>>,
    shape: Idx,
}

impl Index<Idx> for Map {
    type Output = PipeKind;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}

impl Map {
    fn from_lines(input: String) -> Self {
        let data: Vec<Vec<PipeKind>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(PipeKind::from_char)
                    .collect::<Vec<PipeKind>>()
            })
            .collect();
        let shape = (data.len(), data.first().unwrap().len());

        Self { data, shape }
    }

    fn distance_map(&self) -> Vec<Vec<i64>> {
        let mut map = Vec::new();
        for _ in 0..self.shape.0 {
            map.push(vec![-1; self.shape.1])
        }
        map
    }

    fn idx_direction(&self, idx: Idx, direction: &Direction) -> Option<Idx> {
        match direction {
            Direction::North => {
                if idx.0 == 0 {
                    None
                } else {
                    Some((idx.0 - 1, idx.1))
                }
            }
            Direction::East => {
                if idx.1 == self.shape.1 - 1 {
                    None
                } else {
                    Some((idx.0, idx.1 + 1))
                }
            }
            Direction::South => {
                if idx.0 == self.shape.0 - 1 {
                    None
                } else {
                    Some((idx.0 + 1, idx.1))
                }
            }
            Direction::West => {
                if idx.1 == 0 {
                    None
                } else {
                    Some((idx.0, idx.1 - 1))
                }
            }
        }
    }
    fn find_connecting_pipes(&self, idx: Idx) -> Vec<(Pipe, Direction)> {
        let pipe = &self[idx];
        let mut pipes = Vec::new();
        for dir in pipe.directions() {
            if let Some(didx) = self.idx_direction(idx, &dir) {
                let pipe = &self[didx];
                if pipe
                    .directions()
                    .iter()
                    .find_map(|d| if d.oposite() == dir { Some(()) } else { None })
                    .is_some()
                {
                    pipes.push((
                        Pipe {
                            idx: didx,
                            kind: *pipe,
                        },
                        dir,
                    ));
                }
            }
        }

        pipes
    }

    fn find_start(&self) -> Option<Idx> {
        self.data.iter().enumerate().find_map(|(ridx, row)| {
            row.iter().enumerate().find_map(|(cidx, pipe)| {
                if pipe == &PipeKind::S {
                    Some((ridx, cidx))
                } else {
                    None
                }
            })
        })
    }
    fn next_pipe(&self, pipe: &Pipe, old_direction: &Direction) -> Option<(Pipe, Direction)> {
        let idx = pipe.idx;
        if let Some(direction) = pipe
            .kind
            .directions()
            .iter()
            .filter(|&dir| dir != &old_direction.oposite())
            .nth(0)
        {
            if let Some(idx) = self.idx_direction(idx, direction) {
                let kind = self[idx];
                kind.directions().iter().find_map(|d| {
                    if &d.oposite() == direction {
                        Some((Pipe { idx, kind }, (*direction).to_owned()))
                    } else {
                        None
                    }
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}

fn write_dist(idx: Idx, dist: i64, map: &mut [Vec<i64>]) -> bool {
    let val = map[idx.0][idx.1];
    if val > dist || val == -1 {
        map[idx.0][idx.1] = dist;
        true
    } else {
        false
    }
}

pub fn part1(input: String) -> i64 {    let map = Map::from_lines(input);

    let start = map.find_start().unwrap();

    let mut pipes = map.find_connecting_pipes(start);

    let mut dist_map = map.distance_map();
    let mut distance = 1;
    while !pipes.is_empty() {
        pipes = pipes
            .iter()
            .filter_map(|(pipe, dir)| {
                // we write to the distance map and return None if it was not written
                if write_dist(pipe.idx, distance, &mut dist_map) {
                    map.next_pipe(pipe, dir)
                } else {
                    None
                }
            })
            .collect();
        distance += 1;
    }
    // dist_map);
    let solution = dist_map.iter().flatten().max().unwrap();
    *solution as i64
}

fn get_rot(olddir: &Direction, dir: &Direction) -> i64 {
    match olddir {
        Direction::North => match dir {
            Direction::North => 0,
            Direction::East => 1,
            Direction::West => -1,
            _ => panic!("wrong direction"),
        },
        Direction::East => match dir {
            Direction::East => 0,
            Direction::North => -1,
            Direction::South => 1,
            _ => panic!("wrong direction"),
        },
        Direction::South => match dir {
            Direction::South => 0,
            Direction::West => 1,
            Direction::East => -1,
            _ => panic!("wrong direction"),
        },
        Direction::West => match dir {
            Direction::West => 0,
            Direction::North => 1,
            Direction::South => -1,
            _ => panic!("wrong direction"),
        },
    }
}

pub fn part2(input: String) -> i64 {    let map = Map::from_lines(input.clone());

    let start = map.find_start().unwrap();

    let pipes = map.find_connecting_pipes(start);
    assert!(pipes.len() == 2);
    let mut start_dirs = [pipes[0].1, pipes[1].1];
    start_dirs.sort();
    let start_kind = PipeKind::from_directions(&start_dirs);

    let pipidx = 0;
    let (mut pipe, mut dir) = pipes[pipidx];

    let mut loop_map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    loop_map[start.0][start.1] = start_kind.to_char();

    // find the total rotations for determining the orientation of the loop
    let mut rot = 0;

    while pipe.idx != start {
        loop_map[pipe.idx.0][pipe.idx.1] = 'x';
        let olddir = dir;
        (pipe, dir) = map.next_pipe(&pipe, &dir).unwrap();
        rot += get_rot(&olddir, &dir);
    }

    // make sure we run clockwise by using the different starting dir if rot < 0
    let (mut pipe, mut dir) = if rot > 0 {
        pipes[pipidx]
    } else {
        pipes[1 - pipidx]
    };

    let mut insides = HashSet::new();

    while pipe.idx != start {
        if let Some(idx) = map.idx_direction(pipe.idx, &dir.right()) {
            if loop_map[idx.0][idx.1] != 'x' && loop_map[idx.0][idx.1] != 'I' {
                loop_map[idx.0][idx.1] = 'I';
                insides.insert(idx);
            }
        }
        (pipe, dir) = map.next_pipe(&pipe, &dir).unwrap();
    }

    let mut active = insides.clone();
    // for each inside we look at all the neighbors an mark them if not
    while active.iter().len() > 0 {
        let idxs: Vec<Idx> = active.iter().copied().collect();
        active.clear();
        for idx in idxs {
            for direction in ALL_DIRECTIONS {
                if let Some(next_idx) = map.idx_direction(idx, &direction) {
                    let next_val = loop_map[next_idx.0][next_idx.1];
                    if next_val != 'I' && next_val != 'x' {
                        insides.insert(next_idx);
                        active.insert(next_idx);
                        loop_map[next_idx.0][next_idx.1] = 'I';
                    }
                }
            }
        }
    }

    let solution = insides.len();
    solution as i64
}
