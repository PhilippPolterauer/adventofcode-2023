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
}

pub fn part1(input: &str) -> i64 {
    let mats: Vec<CharMatrix> = input.split("\n\n").map(CharMatrix::from_string).collect();

    // test vertical
    let mut solution = 0;
    for mat in mats {
        for idx in 0..(mat.width - 1) {
            let mut mirror = true;
            let mut offset = 0;
            while mirror {
                if let Some((a, b)) = mat.col_pair(idx, offset) {
                    mirror = a == b;
                    // dbg!(&a, &b);
                } else {
                    // if we get here we must have been a mirror, with size offset-1
                    break;
                };
                offset += 1;
            }
            if mirror {
                solution += idx + 1;
            }
        }

        for idx in 0..(mat.height() - 1) {
            let mut mirror = true;
            let mut offset = 0;
            while mirror {
                if let Some((a, b)) = mat.row_pair(idx, offset) {
                    mirror = a == b;
                    // dbg!(&a, &b);
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
    }
    dbg!(solution);
    0
}
pub fn part2(_input: &str) -> i64 {
    0
}
