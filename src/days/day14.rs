use std::{collections::HashMap, fmt::Debug};

#[derive(Debug, Clone, PartialEq, Copy, Hash, Eq)]
enum Element {
    Empty,
    Round,
    Cube,
}
impl Element {
    fn from_char(char: char) -> Self {
        match char {
            '.' => Element::Empty,
            'O' => Element::Round,
            '#' => Element::Cube,
            _ => panic!(),
        }
    }
    fn as_char(&self) -> char {
        match self {
            Element::Empty => '.',
            Element::Round => 'O',
            Element::Cube => '#',
        }
    }
}

pub fn part1(input: &str) -> i64 {
    let mut board = Board::from_string(input);
    board.tilt();
    board.compute_load()
}
#[derive(Hash, Eq, PartialEq, Clone)]
struct Board {
    data: Vec<Element>,
    width: usize,
}
impl Board {
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
    fn cols(&self) -> Vec<Vec<Element>> {
        let mut res = Vec::new();
        for idx in 0..self.width {
            if let Some(col) = self.col(idx) {
                res.push(col)
            }
        }
        res
    }
    fn row(&self, idx: usize) -> Option<Vec<Element>> {
        if idx >= self.height() {
            None
        } else {
            let start = idx * self.width;
            let stop = (idx + 1) * self.width;
            Some(self.data[start..stop].to_vec())
        }
    }
    fn col(&self, idx: usize) -> Option<Vec<Element>> {
        if idx >= self.width {
            None
        } else {
            let step = self.width;
            Some(self.data.iter().skip(idx).step_by(step).copied().collect())
        }
    }
    fn rows(&self) -> Vec<Vec<Element>> {
        let mut res = Vec::new();
        for idx in 0..self.height() {
            if let Some(row) = self.row(idx) {
                res.push(row);
            }
        }
        res
    }

    fn tilt(&mut self) -> &Self {
        let nrows = self.height();
        let mut newcols = Vec::new();
        for col in self.cols() {
            let mut newcol = vec![Element::Empty; nrows];
            let mut iter = col.iter().copied().enumerate();
            let mut idx = 0;
            while let Some((eidx, elem)) = iter.find(|(_, e)| e != &Element::Empty) {
                match elem {
                    Element::Round => {
                        newcol[idx] = Element::Round;
                        idx += 1;
                    }
                    Element::Cube => {
                        newcol[eidx] = Element::Cube;
                        idx = eidx + 1
                    }
                    Element::Empty => (),
                }
            }
            newcols.push(newcol);
        }
        self.data = data_from_cols(&newcols);
        self
    }
    fn rot(&mut self) -> &Self {
        // we rotate by converting to rows reversing them and constructing data from cols
        let height = self.height();
        let newcols: Vec<Vec<Element>> = self.rows().iter().rev().cloned().collect();

        self.width = height;
        self.data = data_from_cols(&newcols);
        self
    }
    fn cycle(&mut self) -> &Self {
        for _ in 0..4 {
            self.tilt();
            self.rot();
        }
        self
    }
    fn compute_load(&self) -> i64 {
        let mut load = 0;
        for (w, row) in self.rows().iter().rev().enumerate() {
            load += (w + 1) * row.iter().filter(|e| e == &&Element::Round).count();
        }
        load as i64
    }
}
impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.rows() {
            let data = line.iter().map(|c| c.as_char()).collect::<String>();
            f.write_str(&(data + "\n"))?
        }
        f.write_str("\n")
    }
}
fn data_from_cols(cols: &Vec<Vec<Element>>) -> Vec<Element> {
    let nrows = cols[0].len();
    let mut data = Vec::new();
    for i in 0..nrows {
        for c in cols {
            data.push(c[i]);
        }
    }
    data
}
pub fn part2(input: &str) -> i64 {
    let mut board = Board::from_string(input);
    const NCYCLES: usize = 1000000000;
    let mut set = HashMap::new();

    let mut i = 0;

    while !set.contains_key(&board) {
        set.insert(board.clone(), i);
        board.cycle();
        // println!("{:?}", &board);
        i += 1;
    }
    let rem = (NCYCLES - i) % (i - set.get(&board).unwrap());

    for _ in 0..rem {
        board.cycle();
    }
    board.compute_load()
}
