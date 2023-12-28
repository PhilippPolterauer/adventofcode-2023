use nalgebra::Vector3;
use std::ops::{Add, Mul};
use std::ops::{Index, IndexMut};
pub fn parse_vec3(string: &str) -> Vector3<i64> {
    Vector3::from_iterator(
        string
            .trim()
            .split(',')
            .filter_map(|part| part.trim().parse::<i64>().ok()),
    )
}
pub fn load_file(day: i32, part: i32, runtest: bool, data_path: &str) -> String {
    let teststr = if runtest { "test_" } else { "" };

    let path = std::format!("{data_path}/day{day}/{teststr}input{part}.txt");
    println!("loading data from '{}'", path);
    std::fs::read_to_string(path).unwrap()
}

#[derive(PartialEq, Clone, Copy, Hash, Eq, Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}
impl Direction {
    pub fn from_char(value: char) -> Self {
        match value {
            'U' => Direction::Up,
            'R' => Direction::Right,
            'D' => Direction::Down,
            'L' => Direction::Left,
            _ => panic!("should not happen!"),
        }
    }
    pub fn opposite(&self) -> Self {
        use Direction::*;
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}

pub const ALL_DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct MatrixIdx {
    pub row: i64,
    pub col: i64,
}

// fn get_rot(last: &Direction, next: &Direction) -> i64 {
//     if last == next {
//         return 0;
//     }

//     match (last, next) {
//         (Direction::Up, Direction::Right)
//         | (Direction::Right, Direction::Down)
//         | (Direction::Down, Direction::Left)
//         | (Direction::Left, Direction::Up) => 1,
//         (Direction::Right, Direction::Up)
//         | (Direction::Down, Direction::Right)
//         | (Direction::Left, Direction::Down)
//         | (Direction::Up, Direction::Left) => -1,
//         _ => panic!("should not happen!"),
//     }
// }
// impl Direction {
//     pub fn get_delta(&self) -> MatrixIdx {
//         match self {
//             Direction::Up => MatrixIdx { row: -1, col: 0 },
//             Direction::Down => MatrixIdx { row: 1, col: 0 },
//             Direction::Left => MatrixIdx { row: 0, col: -1 },
//             Direction::Right => MatrixIdx { row: 0, col: 1 },
//         }
//     }
//     pub fn right(&self) -> Self {
//         match self {
//             Direction::Up => Direction::Right,
//             Direction::Down => Direction::Left,
//             Direction::Left => Direction::Up,
//             Direction::Right => Direction::Down,
//         }
//     }
// }

pub trait FromChar {
    fn from_char(char: &char) -> Self;
    fn default() -> Self;
}
pub trait MatrixElement: FromChar + Clone + PartialEq {}

impl FromChar for i64 {
    fn from_char(char: &char) -> Self {
        *char as i64 - '0' as i64
    }
    fn default() -> Self {
        0
    }
}
impl MatrixElement for i64 {}
#[derive(Debug)]
pub struct Matrix<T>
where
    T: MatrixElement,
{
    data: Vec<T>,
    width: i64,
}
impl<T> Index<MatrixIdx> for Matrix<T>
where
    T: MatrixElement,
{
    type Output = T;

    fn index(&self, index: MatrixIdx) -> &Self::Output {
        &self.data[self.linidx(&index)]
    }
}

impl<T> IndexMut<MatrixIdx> for Matrix<T>
where
    T: MatrixElement,
{
    fn index_mut(&mut self, index: MatrixIdx) -> &mut Self::Output {
        self.data.index_mut(self.linidx(&index))
    }
}

impl Mul<i64> for MatrixIdx {
    type Output = MatrixIdx;
    fn mul(self, rhs: i64) -> Self::Output {
        MatrixIdx {
            row: self.row * rhs,
            col: self.col * rhs,
        }
    }
}
impl Add<i64> for MatrixIdx {
    type Output = MatrixIdx;
    fn add(self, rhs: i64) -> Self::Output {
        MatrixIdx {
            row: self.row + rhs,
            col: self.col + rhs,
        }
    }
}
impl Add<MatrixIdx> for MatrixIdx {
    type Output = MatrixIdx;
    fn add(self, rhs: MatrixIdx) -> Self::Output {
        MatrixIdx {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}
impl Add<&MatrixIdx> for MatrixIdx {
    type Output = MatrixIdx;
    fn add(self, rhs: &MatrixIdx) -> Self::Output {
        MatrixIdx {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

impl<T> Matrix<T>
where
    T: MatrixElement,
{
    fn linidx(&self, idx: &MatrixIdx) -> usize {
        (idx.row * self.width + idx.col) as usize
    }
    pub fn from_string(input: &str) -> Self {
        let mut data = Vec::new();
        let mut width = 0;
        for line in input.lines() {
            width = line.len() as i64;
            for c in line.chars() {
                data.push(T::from_char(&c));
            }
        }
        Self { data, width }
    }
    fn idx_from_lin(&self, linidx: usize) -> MatrixIdx {
        MatrixIdx {
            row: linidx as i64 / self.width,
            col: linidx as i64 % self.width,
        }
    }
    // pub fn neighbours<'a>(&'a self, position: &MatrixIdx, condition: fn(&T) -> bool) -> Vec<&'a T> {
    //     ALL_DIRECTIONS
    //         .iter()
    //         .filter_map(|dir| {
    //             self.getnext(position, dir)
    //                 .and_then(|elem| condition(elem).then_some(elem))
    //         })
    //         .collect()
    // }
    pub fn neighbour_idzs_filt<'a>(
        &'a self,
        idx: &MatrixIdx,
        condition: fn(&T) -> bool,
    ) -> Vec<MatrixIdx> {
        ALL_DIRECTIONS
            .iter()
            .filter_map(|dir| {
                self.next(idx, dir)
                    .and_then(|idx| condition(&self[idx]).then_some(idx))
            })
            .collect()
    }
    pub fn neighbour_idzs_dir(&self, idx: &MatrixIdx) -> Vec<(MatrixIdx, Direction)> {
        ALL_DIRECTIONS
            .iter()
            .filter_map(|dir| self.next(idx, dir).map(|idx| (idx, *dir)))
            .collect()
    }
    pub fn neighbour_idzs(&self, idx: &MatrixIdx) -> Vec<MatrixIdx> {
        ALL_DIRECTIONS
            .iter()
            .filter_map(|dir| self.next(idx, dir))
            .collect()
    }
    // fn empty(nrows: usize, ncols: usize) -> Self {
    //     Self {
    //         data: vec![T::default(); nrows * ncols],
    //         width: ncols as i64,
    //     }
    // }
    // fn row(&self, idx: i64) -> Option<Vec<T>> {
    //     if idx >= self.height() {
    //         None
    //     } else {
    //         let start = idx * self.width;
    //         let stop = (idx + 1) * self.width;
    //         Some(self.data[start as usize..stop as usize].to_owned())
    //     }
    // }
    // // fn rows(&self) -> FilterMap<std::ops::Range<i64>,Vec<T>> {
    // //     (0..self.height()).filter_map(|idx| self.row(idx))
    // // }
    pub fn height(&self) -> i64 {
        self.data.len() as i64 / self.width
    }
    pub fn width(&self) -> i64 {
        self.width
    }
    pub fn next(&self, idx: &MatrixIdx, direction: &Direction) -> Option<MatrixIdx> {
        let n = self.height() - 1;
        let m = self.width - 1;
        let MatrixIdx { row, col } = idx;
        if (row == &0 && direction == &Direction::Up)
            || (row == &n && direction == &Direction::Down)
            || (col == &0 && direction == &Direction::Left)
            || (col == &m && direction == &Direction::Right)
        {
            None
        } else {
            let (row, col) = match direction {
                Direction::Down => (row + 1, *col),
                Direction::Up => (row - 1, *col),
                Direction::Left => (*row, col - 1),
                Direction::Right => (*row, col + 1),
            };
            Some(MatrixIdx { row, col })
        }
    }
    pub fn next_unchecked(&self, idx: &MatrixIdx, direction: &Direction) -> MatrixIdx {
        let MatrixIdx { row, col } = idx;

        let (row, col) = match direction {
            Direction::Down => (row + 1, *col),
            Direction::Up => (row - 1, *col),
            Direction::Left => (*row, col - 1),
            Direction::Right => (*row, col + 1),
        };
        MatrixIdx { row, col }
    }
    fn shape(&self) -> (i64, i64) {
        (self.height(), self.width)
    }
    pub fn get_wrapped(&self, idx: &MatrixIdx) -> &T {
        let MatrixIdx { row, col } = idx;
        let (height, width) = self.shape();

        &self[MatrixIdx {
            row: row.rem_euclid(height),
            col: col.rem_euclid(width),
        }]
    }
    // fn getnext(&self, idx: &MatrixIdx, direction: &Direction) -> Option<&T> {
    //     self.next(idx, direction).and_then(|idx| Some(&self[idx]))
    // }
    pub fn find(&self, element: &T) -> Option<MatrixIdx> {
        self.data
            .iter()
            .enumerate()
            .find_map(|(idx, p)| (p == element).then_some(self.idx_from_lin(idx)))
    }
    pub fn findall(&self, condition: fn(&T) -> bool) -> Vec<MatrixIdx> {
        self.data
            .iter()
            .enumerate()
            .filter_map(|(idx, p)| condition(p).then_some(self.idx_from_lin(idx)))
            .collect()
    }
}

// fn print_matrix(matrix: &Matrix<i64>) {
//     for i in 0..matrix.height() {
//         let line = matrix.row(i).unwrap();
//         for char in line {
//             print!("{}", char);
//         }
//         println!();
//     }
// }
