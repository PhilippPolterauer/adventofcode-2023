use crate::util::*;

#[derive(Debug, PartialEq)]
struct Instruction {
    direction: Direction,
    distance: i64,
}

fn dig(pos: &MatrixIdx, instruction: &Instruction) -> MatrixIdx {
    let MatrixIdx { row, col } = pos;
    let row = match instruction.direction {
        Direction::Down => row + instruction.distance,
        Direction::Up => row - instruction.distance,
        Direction::Left | Direction::Right => *row,
    };
    let col = match instruction.direction {
        Direction::Left => col - instruction.distance,
        Direction::Right => col + instruction.distance,
        Direction::Up | Direction::Down => *col,
    };
    MatrixIdx { row, col }
}

fn parse_line(line: &str) -> Instruction {
    let mut parts = line.split_whitespace();
    let direction = Direction::from_char(parts.next().unwrap().chars().next().unwrap());
    let distance = parts.next().unwrap().parse::<i64>().unwrap();

    Instruction {
        direction,
        distance,
    }
}

fn parse_line2(line: &str) -> Instruction {
    let mut parts = line.split_whitespace();
    let color_str: String = parts.nth(2).unwrap().chars().skip(2).take(5).collect();
    let dir = line.chars().rev().nth(1).unwrap();
    let direction = match dir {
        '0' => Direction::Right,
        '1' => Direction::Down,
        '2' => Direction::Left,
        '3' => Direction::Up,
        _ => panic!("should not happen"),
    };
    let distance = i64::from_str_radix(&color_str, 16).unwrap();
    Instruction {
        direction,
        distance,
    }
}

fn sholace(idzs: &[MatrixIdx]) -> i64 {
    // A = 1/2 Sum (yi + yi1)*(xi - xi1)
    let mut area = 0;
    let mut curr = idzs[0];
    for next in idzs.iter().skip(1) {
        let xi = curr.col;
        let yi = curr.row;
        let xii = next.col;
        let yii = next.row;
        area += xi * yii - xii * yi;

        curr = *next;
    }
    area / 2
}

pub fn part1(input: &str) -> i64 {
    let instructions: Vec<Instruction> = input.lines().map(parse_line).collect();

    let mut position = MatrixIdx { row: 0, col: 0 };
    let mut positions = vec![position];
    for instruction in instructions.iter() {
        position = dig(&position, instruction);
        positions.push(position);
    }

    let a = sholace(&positions);
    let b = instructions
        .iter()
        .fold(0, |peri, inst| peri + inst.distance);
    a + b / 2 + 1
}
pub fn part2(input: &str) -> i64 {
    let instructions: Vec<Instruction> = input.lines().map(parse_line2).collect();
    let mut position = MatrixIdx { row: 0, col: 0 };
    let mut positions = vec![position];
    for instruction in instructions.iter() {
        position = dig(&position, instruction);
        positions.push(position);
    }
    let a = sholace(&positions);
    let b = instructions
        .iter()
        .fold(0, |peri, inst| peri + inst.distance);
    a + b / 2 + 1
}
