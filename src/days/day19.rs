use std::cmp::{max, min};
use std::collections::HashMap;
enum WorkflowResult<'a> {
    Accept,
    Reject,
    Next(&'a str),
}

#[derive(Debug, Clone, Copy)]
struct Range {
    min: i64,
    max: i64,
}
impl Range {
    fn new() -> Range {
        Range { min: 1, max: 4000 }
    }
    fn apply(&self, op: &char, limit: &i64) -> (Self, Self) {
        match op {
            '<' => (
                Range {
                    min: self.min,
                    max: limit - 1,
                },
                Range {
                    min: *limit,
                    max: self.max,
                },
            ),
            '>' => (
                Range {
                    min: limit + 1,
                    max: self.max,
                },
                Range {
                    min: self.min,
                    max: *limit,
                },
            ),
            _ => panic!("wrong operator {op}, expected < or >"),
        }
    }
    fn len(&self) -> i64 {
        if self.max >= self.min {
            self.max - self.min + 1
        } else {
            0
        }
    }
    fn intersect(&self, other: &Range) -> Range {
        Range {
            min: max(self.min, other.min),
            max: min(self.max, other.max),
        }
    }
}
#[derive(Debug, Clone, Copy)]

struct PartRange {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}
impl PartRange {
    fn new() -> Self {
        Self {
            x: Range::new(),
            m: Range::new(),
            a: Range::new(),
            s: Range::new(),
        }
    }
    fn apply(&self, rule: &Rule) -> (Self, Self) {
        let Rule {
            member, op, limit, ..
        } = rule;
        let mut positive = *self;
        let mut negative = *self;
        match member {
            'x' => (positive.x, negative.x) = self.x.apply(op, limit),
            'm' => (positive.m, negative.m) = self.m.apply(op, limit),
            'a' => (positive.a, negative.a) = self.a.apply(op, limit),
            's' => (positive.s, negative.s) = self.s.apply(op, limit),
            _ => panic!("wrong member {member}, expected x, m, a, s"),
        };
        (positive, negative)
    }
    fn size(&self) -> i64 {
        self.x.len() * self.m.len() * self.a.len() * self.s.len()
    }
    fn intersect(&self, other: &Self) -> Self {
        Self {
            x: self.x.intersect(&other.x),
            m: self.m.intersect(&other.m),
            a: self.a.intersect(&other.a),
            s: self.s.intersect(&other.s),
        }
    }
}

#[derive(Debug)]
struct Rule {
    member: char,
    op: char,
    limit: i64,
    target: String,
}
type Workflows = HashMap<String, Workflow>;
type Part = Vec<i64>;

#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
    default: String,
}

fn parse_rule(rule: &str) -> Rule {
    let (rule, target) = rule.split_once(':').unwrap();
    let target = target.to_owned();
    let mut chars = rule.chars();
    let member = chars.next().unwrap();
    let op = chars.next().unwrap();
    let limit = chars.collect::<String>().parse::<i64>().unwrap();
    Rule {
        member,
        op,
        limit,
        target,
    }
}

fn parse_workflow(line: &str) -> (String, Workflow) {
    let (name, rest) = line.split_once('{').unwrap();
    let mut iter = rest.split(',').rev();
    let default = iter.next().unwrap().replace('}', "");
    let mut rules = Vec::new();
    for rule_str in iter.rev() {
        rules.push(parse_rule(rule_str))
    }

    (name.to_owned(), Workflow { rules, default })
}

fn parse_part(line: &str) -> Part {
    let line = line.replace(['{', '}'], "");
    line.split(',')
        .map(|member| {
            let (_, num) = member.split_once('=').unwrap();
            num.parse::<i64>().unwrap()
        })
        .collect()
}

fn parse_input(input: &str) -> (Workflows, Vec<Part>) {
    let mut workflows = HashMap::new();
    let (workflow_lines, object_lines) = input.split_once("\n\n").unwrap();
    for line in workflow_lines.lines() {
        if line.is_empty() {
            break;
        }
        let (name, workflow) = parse_workflow(line);
        workflows.insert(name, workflow);
    }

    // parse parts
    let mut parts = Vec::new();

    for line in object_lines.lines() {
        parts.push(parse_part(line));
    }

    (workflows, parts)
}
fn apply_rule<'a>(rule: &'a Rule, part: &Part) -> Option<&'a str> {
    let Rule {
        member,
        op,
        limit,
        target,
    } = rule;
    let num = match member {
        'x' => part[0],
        'm' => part[1],
        'a' => part[2],
        's' => part[3],
        _ => panic!("wrong member {member}, expected x, m, a, s"),
    };
    let test = match op {
        '<' => &num < limit,
        '>' => &num > limit,
        _ => panic!("wrong operator {op}, expected < or >"),
    };
    if test {
        Some(target)
    } else {
        None
    }
}
fn run_workflow<'a>(workflow: &'a Workflow, part: &Part) -> WorkflowResult<'a> {
    let Workflow { rules, default } = workflow;
    let mut result: &str = default;

    for rule in rules {
        if let Some(target) = apply_rule(rule, part) {
            result = target;
            break;
        }
    }
    match result {
        "A" => WorkflowResult::Accept,
        "R" => WorkflowResult::Reject,
        _ => WorkflowResult::Next(result),
    }
}

fn run_workflow_range<'a>(workflow: &'a Workflow, part: &PartRange) -> Vec<(&'a str, PartRange)> {
    let Workflow { rules, default } = workflow;
    let mut nexts: Vec<(&'a str, PartRange)> = Vec::new();
    let mut neg = *part;
    for rule in rules {
        let (positive, negative) = neg.apply(rule);
        nexts.push((&rule.target, positive));
        neg = negative;
    }
    nexts.push((default, neg));
    nexts
}

pub fn part1(input: &str) -> i64 {
    let (workflows, parts) = parse_input(input);
    let mut sum = 0;
    dbg!(&workflows, &parts);
    for part in parts {
        let mut workflow = &workflows["in"];
        let mut result = run_workflow(workflow, &part);
        while let WorkflowResult::Next(next) = result {
            workflow = &workflows[next];
            result = run_workflow(workflow, &part);
        }
        sum += match result {
            WorkflowResult::Accept => part.iter().sum::<i64>(),
            _ => 0,
        };
    }
    sum
}
pub fn part2(input: &str) -> i64 {
    let (workflows, _) = parse_input(input);

    let mut parts = vec![("in", PartRange::new())];
    let mut accepted = Vec::new();
    while !parts.is_empty() {
        // let nexts = run_workflow_range(workflow, part).
        let mut nexts = Vec::new();
        for (name, part) in parts {
            let workflow = &workflows[name];

            for (name, part) in run_workflow_range(workflow, &part) {
                match name {
                    "A" => {
                        accepted.push(part);
                    }
                    "R" => (),
                    _ => nexts.push((name, part)),
                }
            }
        }
        parts = nexts;
    }

    let mut sum = 0;
    for partrange in &accepted {
        sum += partrange.size();
    }
    // test for intersections -> appearantly there are no intersections
    let n = accepted.len();
    for (start, this) in accepted[0..n].iter().enumerate() {
        for other in &accepted[start + 1..] {
            assert_eq!(this.intersect(other).size(), 0);
        }
    }

    sum
}
