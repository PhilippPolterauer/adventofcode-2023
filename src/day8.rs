use std::collections::{HashMap, HashSet};

type Node = [char; 3];
#[derive(Debug, Clone)]
enum Direction {
    LEFT,
    RIGHT,
}

impl Direction {
    fn from_char(char: char) -> Direction {
        match char {
            'L' => Self::LEFT,
            'R' => Self::RIGHT,
            _ => panic!("should not get here"),
        }
    }
}

trait MNode {
    fn from_string(string: &str) -> Node;
}
impl MNode for Node {
    fn from_string(string: &str) -> Node {
        let temp: Vec<char> = string.chars().collect();
        let mut name: [char; 3] = ['0'; 3];
        for i in 0..3 {
            name[i] = temp[i];
        }
        name
    }
}

#[derive(Debug)]
struct Puzzle {
    directions: Vec<Direction>,
    nodes: HashSet<Node>,
    graph: HashMap<Node, [Node; 2]>,
}
fn parse_line(line: &str) -> (Node, [Node; 2]) {
    let mut split = line.split(" = ");
    let node = Node::from_string(split.next().unwrap());
    let mut children = split.next().unwrap().to_owned();

    children = children.replace("(", "");
    children = children.replace(")", "");
    let mut childrens = children.split(", ");
    let left = Node::from_string(childrens.next().unwrap());
    let right = Node::from_string(childrens.next().unwrap());
    (node, [left, right])
}

impl Puzzle {
    fn parse(input: String) -> Puzzle {
        let mut lines = input.lines();

        let directions: Vec<Direction> = lines
            .next()
            .unwrap()
            .chars()
            .map(Direction::from_char)
            .collect();

        lines.next();
        let mut nodes = HashSet::new();
        let mut graph = HashMap::new();
        for line in lines {
            let (node, [left, right]) = parse_line(line);
            nodes.insert(node);
            nodes.insert(left);
            nodes.insert(right);
            graph.insert(node, [left, right]);
        }

        dbg!(&graph);

        Puzzle {
            directions,
            nodes,
            graph,
        }
    }
    fn traverse(&self, start: &Node, stop: &Node) -> usize {
        let ndir = self.directions.len();
        let mut current = start;
        let mut step = 0;
        while current != stop {
            let direction = &self.directions[step % ndir];
            current = self.next(current, direction).unwrap();
            dbg!(&current);
            step += 1;
        }
        step
    }
    fn traverse2(&self, start: &Node) -> usize {
        let ndir = self.directions.len();
        let mut current = start;
        let mut step = 0;
        while !is_goal(&current) {
            let direction = &self.directions[step % ndir];
            current = self.next(current, direction).unwrap();
            step += 1;
        }
        step
    }
    fn next(&self, current: &Node, direction: &Direction) -> Option<&Node> {
        let children = self.graph[current];

        match direction {
            Direction::LEFT => self.get(&children[0]),
            Direction::RIGHT => self.get(&children[1]),
        }
    }
    fn get_node(&self, name: &str) -> Option<&Node> {
        let node = Node::from_string(name);
        self.nodes.get(&node)
    }
    fn get(&self, node: &Node) -> Option<&Node> {
        self.nodes.get(node)
    }

    fn starting_nodes(&self) -> Vec<&Node> {
        self.nodes.iter().filter_map(is_start).collect()
    }
}

fn is_goal(goal: &&Node) -> bool {
    goal[2] == 'Z'
}
fn is_start(start: &Node) -> Option<&Node> {
    if start[2] == 'A' {
        Some(start)
    } else {
        None
    }
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

pub fn part1(input: String) {
    let puzzle = Puzzle::parse(input);
    dbg!(&puzzle);
    let start = puzzle.get_node("AAA").unwrap();
    let stop = puzzle.get_node("ZZZ").unwrap();
    let solution = puzzle.traverse(start, stop);
    dbg!(solution);
}
pub fn part2(input: String) {
    let puzzle = Puzzle::parse(input);
    dbg!(&puzzle);
    let starts = puzzle.starting_nodes();

    let steps: Vec<usize> = starts.iter().map(|start| puzzle.traverse2(start)).collect();

    let mut solution = 1;
    for step in steps {
        solution = lcm(solution, step)
    }

    dbg!(solution);
}
