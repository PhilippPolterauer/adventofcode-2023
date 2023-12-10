use std::collections::{HashMap, HashSet};
struct CharMatrix {
    inner: Vec<Vec<char>>,
    meta: Vec<Vec<u8>>,
}
impl CharMatrix {
    fn from_string(input: String) -> CharMatrix {
        let mut char_matrix: Vec<Vec<char>> = vec![];
        let mut meta = vec![];
        for line in input.lines() {
            let mut chars = vec![];
            for char in line.chars() {
                chars.push(char);
            }
            let n = chars.len();
            char_matrix.push(chars);
            meta.push(vec![0; n])
        }
        Self {
            inner: char_matrix,
            meta,
        }
    }
    fn shape(&self) -> (i32, i32) {
        (self.inner.len() as i32, self.inner[0].len() as i32)
    }
    fn mark_valid(&mut self, row: i32, col: i32) {
        for i in row - 1..row + 2 {
            for j in col - 1..col + 2 {
                if i == row && j == col {
                    continue;
                }
                self.meta[i as usize][j as usize] = 1;
            }
        }
    }
}

pub struct CharIntoIterator {
    chars: Vec<Vec<char>>,
    line: usize,
    row: usize,
}
impl IntoIterator for CharMatrix {
    type Item = char;
    type IntoIter = CharIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        CharIntoIterator {
            chars: self.inner,
            line: 0,
            row: 0,
        }
    }
}

impl Iterator for CharIntoIterator {
    type Item = char;
    fn next(&mut self) -> Option<char> {
        if self.row == self.chars[self.line].len() {
            self.line += 1;
            self.row = 0;
        }
        if self.line < self.chars.len() {
            let row = self.row;
            self.row += 1;
            Some(self.chars[self.line][row])
        } else {
            None
        }
    }
}

struct Neighbours {
    inner: Vec<Vec<char>>,
    row: i32,
    col: i32,
    cnt: i32,
}

impl Iterator for Neighbours {
    type Item = char;
    fn next(&mut self) -> Option<char> {
        if self.cnt == 9 {
            return None;
        }
        let rowoff = self.cnt % 3 - 1;
        let coloff = self.cnt / 3 - 1;
        if self.cnt == 3 {
            // we skip the central character
            self.cnt += 1;
        }
        self.cnt += 1;

        let row = self.row + rowoff;
        let col = self.col + coloff;

        if let Some(line) = self.inner.get(row as usize) {
            if let Some(character) = line.get(col as usize) {
                return Some(*character);
            }
        }

        Some('.')
    }
}

pub fn part1(input: String) -> i64 {    let mut mat = CharMatrix::from_string(input);
    let (rows, cols) = mat.shape();

    for row in 0..rows {
        for col in 0..cols {
            let center = &mat.inner[row as usize][col as usize];
            if !center.is_ascii_digit() && *center != '.' {
                mat.mark_valid(row, col)
            }
        }
    }
    let mut digit = 0;
    let mut valid = false;
    let mut sum = 0;
    for row in 0..rows {
        for col in 0..cols {
            let center = &mat.inner[row as usize][col as usize];
            let meta = &mat.meta[row as usize][col as usize];

            if center.is_ascii_digit() {
                digit = digit * 10 + center.to_string().parse::<i32>().unwrap();
                if meta == &1u8 {
                    valid = true
                }
            } else {
                if valid {
                    sum += digit;
                }

                valid = false;
                digit = 0;
            }
        }
    }
    dbg!(sum) as i64
}
struct LinCharMatrix {
    inner: Vec<char>,
    linelength: usize,
    nlines: usize,
}
impl LinCharMatrix {
    fn from_string(input: String) -> Self {
        let mut inner = vec![];
        let mut linelength = 0;
        let mut nlines = 0;
        for line in input.lines() {
            for char in line.chars() {
                inner.push(char);
            }
            if linelength != 0 {
                assert_eq!(linelength, line.len())
            } else {
                linelength = line.len();
            }
            nlines += 1;
        }

        Self {
            inner,
            linelength,
            nlines,
        }
    }
    fn shape(&self) -> (usize, usize) {
        (self.nlines, self.linelength)
    }
    fn iter(&self) -> std::slice::Iter<'_, char> {
        self.inner.iter()
    }
}

fn neighbor_idzs(shape: (usize, usize), idx: usize) -> Vec<usize> {
    let (nrow, ncol) = shape;
    let crow = idx / ncol;
    let ccol = idx % ncol;

    let w = 1;
    let h = 1;
    let startrow = std::cmp::max(0, crow as u64 - h) as usize;
    let startcol = std::cmp::max(0, ccol as u64 - w) as usize;
    let endrow = std::cmp::min(nrow - 1, crow + h as usize) as usize;
    let endcol = std::cmp::min(ncol - 1, ccol + w as usize) as usize;

    let mut neighbors = Vec::new();
    for row in startrow..=endrow {
        for col in startcol..=endcol {
            if row == crow && col == ccol {
                continue;
            }
            neighbors.push(row * ncol + col);
        }
    }
    neighbors
}
pub fn part2(input: String) -> i64 {    let mat = LinCharMatrix::from_string(input);
    let mut innumber = false;
    let mut number = 0;
    let mut numberidx = 0usize;
    let mut numbers = vec![];
    let mut numbermap = HashMap::new();
    let shape = mat.shape();

    for (idx, char) in mat.iter().enumerate() {
        if char.is_ascii_digit() {
            innumber = true;
            number = number * 10 + (*char as i32 - '0' as i32);
            numbermap.insert(idx, numberidx);
            dbg!(&char);
            dbg!(&idx);
            dbg!(&numberidx);
        } else {
            if innumber {
                dbg!(&number);
                numbers.push(number);
                numberidx += 1;
            }
            number = 0;
            innumber = false;
        }
    }
    dbg!(&numbermap);
    let mut char_iter = mat.inner.iter().enumerate();
    let mut sum = 0;
    while let Some((gear_index, _)) = char_iter.find(|(_u, c)| **c == '*') {
        let mut neighbor_numbers = HashSet::new();
        for idx in neighbor_idzs(shape, gear_index) {
            if let Some(numberidx) = numbermap.get(&idx) {
                neighbor_numbers.insert(numbers[*numberidx] as i64);
            }
        }
        dbg!(&neighbor_numbers);
        if neighbor_numbers.len() == 2 {
            let solution = neighbor_numbers.iter().product::<i64>();
            sum += solution
        }
    }
    dbg!(sum) as i64
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_neighbor_idzs() {
        assert_eq!(
            vec![31, 32, 33, 41, 43, 51, 52, 53],
            neighbor_idzs((10, 10), 42)
        )
    }
}
