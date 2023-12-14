struct CharMatrix {
    data: Vec<char>,
    width: usize,
}
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
                if let Some(ncol) = fun(self, idx + 1 + offset) {
                    Some((col, ncol))
                } else {
                    None
                }
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
    fn find_smudge(
        &self,
        num: usize,
        pair_fun: fn(&CharMatrix, usize, usize) -> Option<(Vec<char>, Vec<char>)>,
    ) -> Option<(usize, usize)> {
        for idx in 0..(num - 1) {
            let mut mirror = true;
            let mut offset = 0;
            while mirror {
                if let Some((a, b)) = pair_fun(self, idx, offset) {
                    mirror = a == b;
                    if !mirror {
                        if let Some((isa, other_idx)) = smudge(a, b) {
                            if isa {
                                return Some((idx - offset, other_idx));
                            } else {
                                return Some((idx + offset + 1, other_idx));
                            }
                        }
                    }
                    // dbg!(&a, &b);
                } else {
                    // if we get here we must have been a mirror, with size offset-1
                    break;
                };
                offset += 1;
            }
        }
        None
    }
    fn find_col(&self) -> Option<usize> {
        for idx in 0..(self.width - 1) {
            let mut mirror = true;
            let mut offset = 0;
            while mirror {
                if let Some((a, b)) = self.col_pair(idx, offset) {
                    mirror = a == b;
                } else {
                    // if we get here we must have been a mirror, with size offset-1
                    break;
                };
                offset += 1;
            }
            if mirror {
                return Some(idx);
            }
        }
        None
    }
    fn find_row(&self) -> Option<usize> {
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
                return Some(idx);
            }
        }
        None
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
    fn print(&self) {
        for row in 0..self.height() {
            for c in self.row(row).unwrap() {
                print!("{}", c);
            }
            print!("\n");
        }
    }
    fn mirror_distance(
        &self,
        idx: usize,
        n: usize,
        pair_fun: fn(&Self, usize, usize) -> Option<(Vec<char>, Vec<char>)>,
    ) -> usize {
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

fn smudge(a: Vec<char>, b: Vec<char>) -> Option<(bool, usize)> {
    if a.iter().zip(b.iter()).filter(|(a, b)| a != b).count() != 1 {
        return None;
    }

    a.iter()
        .zip(b.iter())
        .enumerate()
        .find_map(|(idx, (a, b))| {
            if a != b {
                let isa = a == &'#';
                Some((isa, idx))
            } else {
                None
            }
        })
}

pub fn part1(input: &str) -> i64 {
    let mats: Vec<CharMatrix> = input.split("\n\n").map(CharMatrix::from_string).collect();

    // test vertical
    let mut solution = 0;
    for mat in mats {
        solution += mat.find_solution();
    }
    dbg!(solution)
}
pub fn part2(input: &str) -> i64 {
    let mut mats: Vec<CharMatrix> = input.split("\n\n").map(CharMatrix::from_string).collect();
    let desired_distance = 0;
    // test vertical
    let mut solution = 0;
    for mat in mats.iter() {
        let nrow = mat.height();
        for row in 0..(nrow - 1) {
            let dist = mat.mirror_distance(row, nrow, CharMatrix::row_pair);
            if dist == desired_distance {
                solution += (row + 1) * 100;
                print!("{} ", row + 1);
            }
        }
        let ncol = mat.width;
        for col in 0..(ncol - 1) {
            let dist = mat.mirror_distance(col, ncol, CharMatrix::col_pair);
            if dist == desired_distance {
                solution += col + 1;
                print!("{} ", col + 1);
            }
        }
        println!("");
    }

    dbg!(solution) as i64
}
