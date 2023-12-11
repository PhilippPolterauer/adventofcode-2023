use std::fmt::Debug;

struct Map {
    data: Vec<char>,
    width: usize,
}
impl Map {
    fn col(&self, col: &usize) -> Vec<char> {
        let width = self.width;
        self.data
            .iter()
            .skip(*col)
            .step_by(width)
            .copied()
            .collect()
    }
    fn row(&self, row: &usize) -> Vec<char> {
        let width = self.width;
        self.data[row * width..(row + 1) * width]
            .iter()
            .copied()
            .collect()
    }
    fn from_str(input: &str) -> Self {
        let mut data = Vec::new();
        let mut width = 0;
        for line in input.lines() {
            data.append(&mut line.chars().collect());
            if width == 0 {
                width = line.len();
            } else {
                assert_eq!(width, line.len());
            }
        }
        Self { data, width }
    }
    fn empty_rows(&self) -> Vec<usize> {
        (0..(self.data.len() / self.width))
            .filter(|idx| self.row(&idx).iter().all(|c| c == &'.'))
            .collect()
    }
    fn empty_cols(&self) -> Vec<usize> {
        (0..self.width)
            .filter(|idx| self.col(&idx).iter().all(|c| c == &'.'))
            .collect()
    }
    fn insert_cols(&mut self, cols: &[usize]) {
        for idx in cols.iter().rev() {
            self.insert_col(idx);
            self.width += 1;
        }
    }
    fn insert_col(&mut self, col: &usize) {
        for idx in (*col..self.data.len()).step_by(self.width).rev() {
            self.data.insert(idx, '.');
        }
    }
    fn insert_rows(&mut self, rows: &[usize]) {
        for idx in rows.iter().rev() {
            self.insert_row(idx);
        }
    }
    fn insert_row(&mut self, row: &usize) {
        let idx = row * self.width;
        for _ in 0..self.width {
            self.data.insert(idx, '.');
        }
    }
    fn idx(&self, idx: &usize) -> (usize, usize) {
        (idx / self.width, idx % self.width)
    }
    fn find_galaxies(&self) -> Vec<(usize, usize)> {
        self.data
            .iter()
            .enumerate()
            .filter_map(|(idx, char)| {
                if char == &'#' {
                    Some(self.idx(&idx))
                } else {
                    None
                }
            })
            .collect()
    }
}
fn dist(a: &(usize, usize), b: &(usize, usize)) -> usize {
    let mut dist = if a.0 > b.0 { a.0 - b.0 } else { b.0 - a.0 };
    dist += if a.1 > b.1 { a.1 - b.1 } else { b.1 - a.1 };
    dist
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = "".to_string();
        for i in 0..self.data.len() {
            if i % self.width == 0 {
                str += "\n";
            }
            str += &self.data[i].to_string();
        }
        f.write_str(&str)
    }
}

pub fn part1(input: &str) -> i64 {
    let mut map = Map::from_str(input);

    let cols2insert = map.empty_cols();
    let rows2insert = map.empty_rows();
    map.insert_cols(&cols2insert);
    map.insert_rows(&rows2insert);

    let galaxies = map.find_galaxies();

    let mut solution = 0;
    for (idx, a) in galaxies.iter().enumerate() {
        for b in galaxies.iter().skip(idx + 1) {
            solution += dist(a, b);
        }
    }

    solution as i64
}
pub fn part2(_input: &str) -> i64 {
    0
}
