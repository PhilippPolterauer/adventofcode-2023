use std::{collections::HashSet, ops::Index};

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

#[derive(Debug)]
struct Pipe {
    idx: Idx,
    kind: PipeKind,
}

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn oposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
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
    fn directions(&self) -> Vec<Direction> {
        match self {
            &PipeKind::NS => vec![Direction::North, Direction::South],
            &PipeKind::EW => vec![Direction::East, Direction::West],
            &PipeKind::NE => vec![Direction::North, Direction::East],
            &PipeKind::NW => vec![Direction::North, Direction::West],
            &PipeKind::SW => vec![Direction::South, Direction::West],
            &PipeKind::SE => vec![Direction::South, Direction::East],
            &PipeKind::G => vec![],
            &PipeKind::S => vec![
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ],
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

    fn get(&self, idx: Idx) -> Option<&PipeKind> {
        if (0..self.shape.0).contains(&idx.0) && (0..self.shape.1).contains(&idx.1) {
            Some(&self[idx])
        } else {
            None
        }
    }

    fn distance_map(&self) -> Vec<Vec<i64>> {
        let mut map = Vec::new();
        for _ in 0..self.shape.0 {
            map.push(vec![-1; self.shape.1])
        }
        map
    }

    // fn north(&self, idx: Idx)-> Option<Pipe>{
    //     let nidx = idx;
    //     self.get(nidx)
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
            if let Some(idx) = self.idx_direction(idx, &direction) {
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

fn write_dist(idx: Idx, dist: i64, map: &mut Vec<Vec<i64>>) -> bool {
    let val = map[idx.0][idx.1] ;
    if val > dist || val == -1 {
        map[idx.0][idx.1] = dist;
        true
    } else {
        false
    }
}

pub fn part1(input: String) {
    let map = Map::from_lines(input);

    let start = map.find_start().unwrap();

    let mut pipes = map.find_connecting_pipes(start);

    let mut dist_map = map.distance_map();
    let mut distance = 1;
    while (&pipes).len() > 0 {
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
    // dbg!(dist_map);
    dbg!(pipes);
    let solution = dist_map.iter().flatten().max().unwrap();
    dbg!(solution);
}
pub fn part2(input: String) {}
