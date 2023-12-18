use std::{collections::HashSet, ops::Index};

#[derive(Clone, Copy, Debug)]
enum Element {
    Empty,
    FMirror,
    BMirror,
    HSplit,
    VSplit,
}
#[derive(PartialEq, Clone, Copy, Hash, Eq, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(PartialEq, Hash, Eq, Clone, Copy, Debug)]
struct NodeIdx {
    row: usize,
    col: usize,
}

impl Element {
    fn from_char(char: char) -> Self {
        match char {
            '.' => Element::Empty,
            '\\' => Element::BMirror,
            '/' => Element::FMirror,
            '-' => Element::HSplit,
            '|' => Element::VSplit,
            _ => panic!(),
        }
    }
    fn as_char(&self) -> char {
        match self {
            Element::Empty => '.',
            Element::BMirror => '\\',
            Element::FMirror => '/',
            Element::HSplit => '-',
            Element::VSplit => '|',
        }
    }
    fn out_dirs(&self, dir: &Direction) -> Vec<Direction> {
        match (self, dir) {
            (Element::Empty, _) => vec![*dir],
            (Element::FMirror, Direction::Right) => vec![Direction::Up],
            (Element::FMirror, Direction::Down) => vec![Direction::Left],
            (Element::FMirror, Direction::Left) => vec![Direction::Down],
            (Element::FMirror, Direction::Up) => vec![Direction::Right],
            (Element::BMirror, Direction::Right) => vec![Direction::Down],
            (Element::BMirror, Direction::Down) => vec![Direction::Right],
            (Element::BMirror, Direction::Left) => vec![Direction::Up],
            (Element::BMirror, Direction::Up) => vec![Direction::Left],
            (Element::HSplit, Direction::Right) => vec![Direction::Right],
            (Element::HSplit, Direction::Down) => vec![Direction::Left, Direction::Right],
            (Element::HSplit, Direction::Left) => vec![Direction::Left],
            (Element::HSplit, Direction::Up) => vec![Direction::Left, Direction::Right],
            (Element::VSplit, Direction::Right) => vec![Direction::Up, Direction::Down],
            (Element::VSplit, Direction::Down) => vec![Direction::Down],
            (Element::VSplit, Direction::Left) => vec![Direction::Up, Direction::Down],
            (Element::VSplit, Direction::Up) => vec![Direction::Up],
        }
    }
}
struct Board {
    data: Vec<Element>,
    width: usize,
}
impl Index<&NodeIdx> for Board {
    type Output = Element;

    fn index(&self, index: &NodeIdx) -> &Element {
        &self.data[self.linidx(index)]
    }
}
impl Board {
    fn linidx(&self, idx: &NodeIdx) -> usize {
        idx.row * self.width + idx.col
    }
    fn from_string(input: &str) -> Self {
        let mut data = Vec::new();
        let mut width = 0;
        for line in input.lines() {
            width = line.len();
            for c in line.chars() {
                data.push(Element::from_char(c));
            }
        }

        Self { data, width }
    }
    fn height(&self) -> usize {
        self.data.len() / self.width
    }
    fn next(&self, (idx, indir): &(NodeIdx, Direction)) -> Vec<(NodeIdx, Direction)> {
        self[idx]
            .out_dirs(indir)
            .iter()
            .map(|odir| (*idx, *odir))
            .collect()
    }
    fn nextin(&self, (idx, outdir): &(NodeIdx, Direction)) -> Option<(NodeIdx, Direction)> {
        let n = self.height() - 1;
        let m = self.width - 1;
        let NodeIdx { row, col } = idx;
        if (row == &0 && outdir == &Direction::Up)
            || (row == &n && outdir == &Direction::Down)
            || (col == &0 && outdir == &Direction::Left)
            || (col == &m && outdir == &Direction::Right)
        {
            None
        } else {
            let (row, col) = match outdir {
                Direction::Down => (row + 1, *col),
                Direction::Up => (row - 1, *col),
                Direction::Left => (*row, col - 1),
                Direction::Right => (*row, col + 1),
            };
            Some((NodeIdx { row, col }, *outdir))
        }
    }
    fn compute_energy(&self, start: (NodeIdx, Direction)) -> i64 {
        let mut visited: HashSet<(NodeIdx, Direction)> = HashSet::from_iter([start]);
        let mut nodes: HashSet<(NodeIdx, Direction)> = HashSet::from(visited.clone());

        while !nodes.is_empty() {
            // these are the nextnodes with indirection after moving with the outdir
            nodes = nodes.iter().flat_map(|node| self.next(node)).collect();
            // at this point nodes contains all the nodes with in directions
            nodes = nodes.iter().filter_map(|n| self.nextin(n)).collect();
            // we remove all nodes that have already bin visited with this direction
            nodes.retain(|n| !visited.contains(n));
            // then we transfer the \ node + outdir
            visited.extend(nodes.iter().copied());
        }

        let visited_idx: HashSet<_> = visited.iter().map(|(idx, _)| idx).collect();
        visited_idx.len() as i64
    }
}
pub fn part1(input: &str) -> i64 {
    let board = Board::from_string(input);
    let start = NodeIdx { row: 0, col: 0 };
    let dir = Direction::Right;
    board.compute_energy((start, dir))
}
pub fn part2(input: &str) -> i64 {
    let board = Board::from_string(input);
    let n = board.height();
    let m = board.width;
    let mut starts = Vec::new();
    for row in 0..n {
        starts.push((NodeIdx { row, col: 0 }, Direction::Right));
        starts.push((NodeIdx { row, col: m - 1 }, Direction::Right));
    }
    for col in 0..m {
        starts.push((NodeIdx { row: 0, col }, Direction::Down));
        starts.push((NodeIdx { row: n - 1, col }, Direction::Up));
    }
    starts.iter().map(|&node| board.compute_energy(node)).max().unwrap() as i64
}
