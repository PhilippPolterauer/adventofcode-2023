use std::collections::HashMap;
#[derive(Debug)]
struct Signal {
    sender: usize,
    target: usize,
    level: bool,
}
#[derive(Debug)]
enum Module {
    FlipFlop(FlipFlop),
    BroadCaster,
    Conjunction(Conjunction),
    Untyped,
}
#[derive(Debug)]
struct Conjunction {
    levels: HashMap<usize, bool>,
}
impl Conjunction {
    fn from_inputs(inputs: &[usize]) -> Self {
        Self {
            levels: inputs.iter().map(|&idx| (idx, false)).collect(),
        }
    }
    fn receive(&mut self, input: usize, pulse: bool) -> Option<bool> {
        if let Some(value) = self.levels.get_mut(&input) {
            *value = pulse;
            Some(!self.levels.values().all(|b| *b))
        } else {
            let keys: Vec<&usize> = self.levels.keys().collect();
            panic!("invalid input idx '{input}', allowed are {keys:#?}");
        }
    }
}

#[derive(Debug)]
struct FlipFlop {
    state: bool,
}
impl FlipFlop {
    fn new() -> Self {
        Self { state: false }
    }
    fn receive(&mut self, pulse: bool) -> Option<bool> {
        if !pulse {
            self.state = !self.state;
            Some(self.state)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Network {
    idxmap: HashMap<String, usize>,
    // names: Vec<String>,
    modules: Vec<Module>,
    outputs: Vec<Vec<usize>>,
    inputs: Vec<Vec<usize>>,
    highcnt: i64,
    lowcnt: i64,
}
impl Network {
    fn step(&mut self, signals: Vec<Signal>) -> Vec<Signal> {
        let mut newsignals = Vec::new();
        for signal in signals {
            if signal.level {
                self.highcnt += 1;
            } else {
                self.lowcnt += 1;
            }
            let idx = signal.target;
            let module = self.modules.get_mut(idx).expect("invalid idx for modules");
            let outputs = &self.outputs[idx];
            let outlevel = match module {
                Module::BroadCaster => Some(signal.level),
                Module::Untyped => None,
                Module::FlipFlop(flipflop) => flipflop.receive(signal.level),
                Module::Conjunction(conjunction) => {
                    conjunction.receive(signal.sender, signal.level)
                }
            };
            if let Some(level) = outlevel {
                let addsignals: Vec<Signal> = outputs
                    .iter()
                    .map(|&target| Signal {
                        target,
                        sender: idx,
                        level,
                    })
                    .collect();
                newsignals.extend(addsignals);
            }
        }
        newsignals
    }
    fn find(&self, name: &str) -> usize {
        self.idxmap[name]
    }
    fn solution(&self) -> i64 {
        self.highcnt * self.lowcnt
    }
    fn print(&self) {
        for m in self.modules.iter() {
            match m {
                // Module::Conjunction(conj) => {
                //     for v in conj.levels.values() {
                //         if *v {
                //             print!("1")
                //         } else {
                //             print!("0")
                //         }
                //     }
                // }
                Module::FlipFlop(flip) => {
                    if flip.state {
                        print!("1")
                    } else {
                        print!("0")
                    }
                }
                _ => (),
            }
        }
        println!("");
    }
}

fn parse_input1(input: &str) -> Network {
    let mut idxmap: HashMap<String, usize> = HashMap::from([("button".to_owned(), 0)]);
    let mut module_kinds = vec!["u"];
    let mut output_names = vec![vec!["broadcaster".to_owned()]];

    let mut names: Vec<String> = vec!["button".to_owned()];

    for (idx, line) in input.lines().enumerate() {
        let (left, right) = line.split_once("->").unwrap();
        let targets: Vec<String> = right.split(',').map(|s| s.trim().to_owned()).collect();

        let (name, kind) = match left.trim() {
            "broadcaster" => ("broadcaster".to_owned(), "broadcaster"),
            left if left.starts_with('%') => {
                let mut name = left.to_owned();
                name.remove(0);
                (name, "%")
            }
            left if left.starts_with('&') => {
                let mut name = left.to_owned();
                name.remove(0);
                (name, "&")
            }
            _ => panic!("should not happen! {left}"),
        };

        idxmap.insert(name.clone(), idx + 1);
        names.push(name);
        module_kinds.push(kind);
        output_names.push(targets);
    }

    let mut idx = names.len();

    for name in output_names.iter().flatten() {
        let key = name.as_str();
        if !idxmap.contains_key(key) {
            // otherwise we insert trivial module
            module_kinds.push("u");
            idxmap.insert(name.clone(), idx);
            names.push(name.clone());

            idx += 1;
        }
    }
    for _ in 0..(idx - output_names.len()) {
        output_names.push(vec![]);
    }

    // convert targets to idx
    let outputs: Vec<Vec<usize>> = output_names
        .iter()
        .map(|outnames| outnames.iter().map(|name| idxmap[name.as_str()]).collect())
        .collect();

    // determine input map
    let mut inputs = vec![Vec::<usize>::new(); outputs.len()];
    for (idx, output) in outputs.iter().enumerate() {
        for oidx in output {
            inputs[*oidx].push(idx)
        }
    }

    // construct modules with known infomration
    let modules: Vec<Module> = module_kinds
        .iter()
        .enumerate()
        .map(|(idx, &kind)| match kind {
            "broadcaster" => Module::BroadCaster,
            "%" => Module::FlipFlop(FlipFlop::new()),
            "&" => Module::Conjunction(Conjunction::from_inputs(&inputs[idx])),
            "u" => Module::Untyped,
            _ => panic!("should not happen! {kind}"),
        })
        .collect();

    Network {
        idxmap,
        modules,
        outputs,
        inputs,
        highcnt: 0,
        lowcnt: 0,
    }
}

pub fn part1(input: &str) -> i64 {
    let mut network = parse_input1(input);
    let target = network.find("broadcaster");
    for _ in 0..1000 {
        let mut signals = vec![Signal {
            sender: 0,
            target,
            level: false,
        }];
        while !signals.is_empty() {
            signals = network.step(signals)
        }
    }

    network.solution()
}
pub fn part2(input: &str) -> i64 {
    let mut network = parse_input1(input);
    let broadcaster = network.find("broadcaster");
    let button = network.find("button");
    let target = network.find("rx");
    
    let inputs = network.inputs[target].clone();
    let inputs = network.inputs[inputs[0]].clone();
    dbg!(&inputs);
    for target in inputs {
        dbg!(&network.modules[target]);
        let mut solution = 0;
        let mut done = false;
        while !done {
            solution += 1;
            let mut signals = vec![Signal {
                sender: button,
                target: broadcaster,
                level: false,
            }];
            while !signals.is_empty() {
                signals = network.step(signals);
            }
            if let Module::Conjunction(conj)= &network.modules[target]{
                done = *conj.levels.values().nth(0).unwrap();
            }
        }
        dbg!(solution);
    }

    0
}
