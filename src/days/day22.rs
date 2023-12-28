use std::collections::{HashMap, HashSet};
use crate::util::parse_vec3;
use nalgebra::Vector3;
type Position = Vector3<i64>;
#[derive(Debug, Clone, Copy, PartialEq)]
struct Brick {
    size: Vector3<i64>,
}
impl Brick {
    fn from_start_end(start: &Position, end: &Position) -> (Position, Self) {
        let pos = start.zip_map(end, std::cmp::min);
        let end = start.zip_map(end, std::cmp::max);
        (
            pos,
            Self {
                size: (end - pos).add_scalar(1),
            },
        )
    }
    fn idzs(&self, start: &Position) -> Vec<(i64, i64, i64)> {
        let mut idzs = Vec::new();
        for zoff in 0..self.size.z {
            let iz = start.z + zoff;
            for yoff in 0..self.size.y {
                let iy = start.y + yoff;
                for xoff in 0..self.size.x {
                    let ix = start.x + xoff;
                    idzs.push((ix, iy, iz))
                }
            }
        }
        idzs
    }
    fn footprint(&self, start: &Position) -> Vec<(i64, i64)> {
        let mut idzs = Vec::new();

        for yoff in 0..self.size.y {
            let iy = start.y + yoff;
            for xoff in 0..self.size.x {
                let ix = start.x + xoff;
                idzs.push((ix, iy))
            }
        }

        idzs
    }
}

#[derive(Debug)]
struct BrickStack {
    occupancy: Vec<HashMap<(i64, i64), usize>>,
    bricks: Vec<Brick>,
    positions: Vec<Position>,
    aboves: Vec<HashSet<usize>>,
    belows: Vec<HashSet<usize>>,
}

impl BrickStack {
    fn new() -> Self {
        Self {
            occupancy: vec![],
            bricks: vec![],
            positions: vec![],
            aboves: vec![],
            belows: vec![],
        }
    }
    fn stack(&mut self, start: Position, brick: Brick) {
        let idzs = brick.footprint(&start);
        let mut pos = start;
        pos.z = 0;
        let mut belows = HashSet::new();
        for (iz, map) in self.occupancy.iter().enumerate().rev() {
            let mut stop = false;
            for xy in &idzs {
                if let Some(&below) = map.get(xy) {
                    belows.insert(below);
                    stop = true
                }
            }
            if stop {
                pos.z = (iz + 1) as i64;
                break;
            }
        }
        self.insert(pos, brick, belows);
    }
    fn insert(&mut self, pos: Position, brick: Brick, belows: HashSet<usize>) {
        let idzs = brick.idzs(&pos);
        self.positions.push(pos);
        let brick_idx = self.bricks.len();
        self.bricks.push(brick);
        self.aboves.push(HashSet::new());

        for below in &belows {
            self.aboves[*below].insert(brick_idx);
        }
        for (x, y, z) in idzs {
            if z as usize >= self.occupancy.len() {
                self.occupancy.push(HashMap::new());
            }
            self.occupancy[z as usize].insert((x, y), brick_idx);
        }
        self.belows.push(belows);
    }
}



fn parse_bricks(input: &str) -> Vec<(Position, Brick)> {
    input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once('~').unwrap();
            Brick::from_start_end(&parse_vec3(a), &parse_vec3(b))
        })
        .collect()
}
fn create_stack(input: &str) -> BrickStack {
    let mut bricks = parse_bricks(input);
    bricks.sort_by_key(|(a, _)| a.z);
    let mut stack = BrickStack::new();
    for (start, brick) in bricks {
        stack.stack(start, brick);
    }
    stack
}
pub fn part1(input: &str) -> i64 {
    let stack = create_stack(input);
    let mut solution = 0;
    for above in stack.aboves.iter() {
        // if every of the above has more than one supports it can be disintegrated
        if above.iter().all(|ab| stack.belows[*ab].len() > 1) {
            solution += 1;
        }
    }
    solution
}

pub fn part2(input: &str) -> i64 {
    let stack = create_stack(input);
    let mut solution = 0i64;

    for (start, above) in stack.aboves.iter().enumerate() {
        let mut above = above.clone();
        let mut delete = HashSet::from([start]);
        while !above.is_empty() {
            let mut next = HashSet::new();
            for idx in above {
                if stack.belows[idx].is_subset(&delete) {
                    delete.insert(idx);
                    next.extend(stack.aboves[idx].iter());
                }
            }
            above = next;
        }
        let cnt = delete.len() - 1;
        // dbg!(start, cnt);
        solution += cnt as i64;
    }
    solution
}
