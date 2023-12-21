#[derive(Debug)]
enum Operation {
    Insert(usize, Lense),
    Remove(usize, String),
}
#[derive(Debug, Clone)]
struct Lense {
    label: String,
    focal: usize,
}
#[derive(Debug, Clone)]
struct Boxes {
    boxes: Vec<Vec<Lense>>,
}
impl Boxes {
    fn apply(&mut self, op: Operation) -> &Self {
        match op {
            Operation::Insert(id, lense) => insert(&mut self.boxes[id], lense),
            Operation::Remove(id, label) => remove(&mut self.boxes[id], &label),
        }
        self
    }
    fn new() -> Self {
        Self {
            boxes: vec![Vec::new(); 256],
        }
    }
    fn focusing_power(&self) -> i64 {
        self.boxes
            .iter()
            .enumerate()
            .map(|(idx, lenses)| focusing_power(idx, lenses))
            .sum()
    }
}
fn focusing_power(idx: usize, lenses: &[Lense]) -> i64 {
    lenses
        .iter()
        .enumerate()
        .map(|(lidx, lense)| (idx + 1) * (lidx + 1) * lense.focal)
        .sum::<usize>() as i64
}
fn find_lense(lbox: &[Lense], label: &str) -> Option<usize> {
    lbox.iter().enumerate().find_map(|(idx, lense)| {
        if lense.label == label {
            Some(idx)
        } else {
            None
        }
    })
}
fn insert(lbox: &mut Vec<Lense>, lense: Lense) {
    if let Some(idx) = find_lense(lbox, &lense.label) {
        lbox.remove(idx);
        lbox.insert(idx, lense)
    } else {
        lbox.push(lense)
    }
}
fn remove(lbox: &mut Vec<Lense>, label: &str) {
    if let Some(idx) = find_lense(lbox, label) {
        lbox.remove(idx);
    }
}

fn parse_operation(text: &str) -> Operation {
    let mut iter = text.chars();
    let mut label = String::new();
    let mut op = ' ';
    for c in iter.by_ref() {
        if c == '=' || c == '-' {
            op = c;
            break;
        } else {
            label.push(c);
        }
    }

    let id = hash(&label);
    if op == '-' {
        Operation::Remove(id, label)
    } else {
        let focal = iter.next().unwrap() as usize - '0' as usize;
        let lense = Lense { focal, label };
        Operation::Insert(id, lense)
    }
}

fn hash(chars: &str) -> usize {
    let mut hash = 0;
    for c in chars.chars() {
        if c == '\n' {
            continue;
        }
        hash += c as usize;
        hash *= 17;
        hash %= 256;
    }
    hash
}

pub fn part1(input: &str) -> i64 {
    let mut solution = 0;
    for split in input.split(',') {
        solution += hash(split);
    }
    solution as i64
}
pub fn part2(input: &str) -> i64 {
    let mut boxes = Boxes::new();
    for split in input.split(',') {
        let op = parse_operation(split);
        boxes.apply(op);
    }
    boxes.focusing_power()
}
