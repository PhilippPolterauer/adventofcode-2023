use std::collections::HashMap;

enum WorkflowResult<'a> {
    Accept,
    Reject,
    Next(&'a str),
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
    let mut chars = rule.chars().into_iter();
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
    let default = iter.next().unwrap().replace("}", "");
    let mut rules = Vec::new();
    for rule_str in iter.rev() {
        rules.push(parse_rule(rule_str))
    }

    (name.to_owned(), Workflow { rules, default })
}

fn parse_part(line: &str) -> Part {
    let line = line.replace("{", "").replace("}", "");
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
pub fn part2(_input: &str) -> i64 {
    0
}
