pub fn load_file(day: i32, part: i32, runtest: bool, data_path: &str) -> String {
    let teststr = if runtest { "test_" } else { "" };

    let path = std::format!("{data_path}/day{day}/{teststr}input{part}.txt");
    println!("loading data from '{}'", path);
    std::fs::read_to_string(path).unwrap()
}
use std::ops::{Add, Mul};
use std::ops::{Index, IndexMut};

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
}

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
pub trait MatrixElement: FromChar + Clone {}

impl FromChar for i64 {
    fn from_char(char: &char) -> Self {
        *char as i64 - '0' as i64
    }
    fn default() -> Self {
        0
    }
}
impl MatrixElement for i64 {}
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
    // fn from_string(input: &str) -> Self {
    //     let mut data = Vec::new();
    //     let mut width = 0;
    //     for line in input.lines() {
    //         width = line.len() as i64;
    //         for c in line.chars() {
    //             data.push(T::from_char(&c));
    //         }
    //     }

    //     Self { data, width }
    // }
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
    // fn height(&self) -> i64 {
    //     self.data.len() as i64 / self.width
    // }
    // fn next(&self, idx: &MatrixIdx, direction: &Direction) -> Option<MatrixIdx> {
    //     let n = self.height() - 1;
    //     let m = self.width - 1;
    //     let MatrixIdx { row, col } = idx;
    //     if (row == &0 && direction == &Direction::Up)
    //         || (row == &n && direction == &Direction::Down)
    //         || (col == &0 && direction == &Direction::Left)
    //         || (col == &m && direction == &Direction::Right)
    //     {
    //         None
    //     } else {
    //         let (row, col) = match direction {
    //             Direction::Down => (row + 1, *col),
    //             Direction::Up => (row - 1, *col),
    //             Direction::Left => (*row, col - 1),
    //             Direction::Right => (*row, col + 1),
    //         };
    //         Some(MatrixIdx { row, col })
    //     }
    // }
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
