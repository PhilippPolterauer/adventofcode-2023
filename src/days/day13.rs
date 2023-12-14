struct CharMatrix {
    data: Vec<char>,
    width: usize,
}
type PairFn = fn(&CharMatrix, usize, usize) -> Option<(Vec<char>, Vec<char>)>;
impl CharMatrix {
    fn height(&self) -> usize {
        self.data.len() / self.width
    }
    fn row(&self, idx: usize) -> Option<Vec<char>> {
        if idx >= self.height() {
            None
        } else {
            let start = idx * self.width;
            let stop = (idx + 1) * self.width;
            Some(self.data[start..stop].to_vec())
        }
    }
    fn col(&self, idx: usize) -> Option<Vec<char>> {
        if idx >= self.width {
            None
        } else {
            let step = self.width;
            Some(self.data.iter().skip(idx).step_by(step).copied().collect())
        }
    }
    fn from_string(input: &str) -> Self {
        let mut data = Vec::new();
        let mut width = 0;
        for line in input.lines() {
            width = line.len();
            for c in line.chars() {
                data.push(c);
            }
        }

        Self { data, width }
    }
    fn pair(
        &self,
        idx: usize,
        offset: usize,
        fun: fn(&Self, usize) -> Option<Vec<char>>,
    ) -> Option<(Vec<char>, Vec<char>)> {
        if offset <= idx {
            if let Some(col) = fun(self, idx - offset) {
                fun(self, idx + 1 + offset).map(|ncol| (col, ncol))
            } else {
                None
            }
        } else {
            None
        }
    }
    fn row_pair(&self, idx: usize, offset: usize) -> Option<(Vec<char>, Vec<char>)> {
        self.pair(idx, offset, Self::row)
    }
    fn col_pair(&self, idx: usize, offset: usize) -> Option<(Vec<char>, Vec<char>)> {
        self.pair(idx, offset, Self::col)
    }

    fn find_solution(&self) -> i64 {
        let mut solution = 0;

        for idx in 0..(self.height() - 1) {
            let mut mirror = true;
            let mut offset = 0;
            while mirror {
                if let Some((a, b)) = self.row_pair(idx, offset) {
                    mirror = a == b;
                } else {
                    // if we get here we must have been a mirror, with size offset-1
                    break;
                };
                offset += 1;
            }
            if mirror {
                solution += (idx + 1) * 100;
            }
        }
        solution as i64
    }
    // fn print(&self) { // allow
    //     for row in 0..self.height() {
    //         for c in self.row(row).unwrap() {
    //             print!("{}", c);
    //         }
    //         print!("\n");
    //     }
    // }
    fn mirror_distance(&self, idx: usize, n: usize, pair_fun: PairFn) -> usize {
        let mut distance = 0;
        let offmax = std::cmp::min(idx + 1, n - idx);
        for offset in 0..offmax {
            if let Some((a, b)) = pair_fun(self, idx, offset) {
                distance += a.iter().zip(b.iter()).filter(|(a, b)| a != b).count();
            } else {
                break;
            }
        }
        distance
    }
}

pub fn part1(input: &str) -> i64 {
    let mats: Vec<CharMatrix> = input.split("\n\n").map(CharMatrix::from_string).collect();

    // test vertical
    let mut solution = 0;
    for mat in mats {
        solution += mat.find_solution();
    }
    solution
}
pub fn part2(input: &str) -> i64 {
    let mats: Vec<CharMatrix> = input.split("\n\n").map(CharMatrix::from_string).collect();
    let desired_distance = 0;
    // test vertical
    let mut solution = 0;
    for mat in mats.iter() {
        let nrow = mat.height();
        for row in 0..(nrow - 1) {
            let dist = mat.mirror_distance(row, nrow, CharMatrix::row_pair);
            if dist == desired_distance {
                solution += (row + 1) * 100;
            }
        }
        let ncol = mat.width;
        for col in 0..(ncol - 1) {
            let dist = mat.mirror_distance(col, ncol, CharMatrix::col_pair);
            if dist == desired_distance {
                solution += col + 1;
            }
        }
    }

    solution as i64
}
