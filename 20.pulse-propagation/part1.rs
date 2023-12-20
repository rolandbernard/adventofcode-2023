use std::{
    collections::{HashMap, VecDeque},
    io::Read,
};

enum Module<'a> {
    Broadcast {
        next: Vec<&'a str>,
    },
    FlipFlop {
        state: bool,
        next: Vec<&'a str>,
    },
    Conjunction {
        state: HashMap<&'a str, bool>,
        next: Vec<&'a str>,
    },
}

fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();
    let mut network = HashMap::new();
    let mut reverse = HashMap::new();
    for line in input.lines() {
        let (name, outputs) = line.split_once(" -> ").unwrap();
        let targets = outputs.split(", ").collect::<Vec<_>>();
        let real_name;
        if name.starts_with('%') {
            real_name = &name[1..];
            network.insert(
                real_name,
                Module::FlipFlop {
                    state: false,
                    next: targets,
                },
            );
        } else if name.starts_with('&') {
            real_name = &name[1..];
            network.insert(
                real_name,
                Module::Conjunction {
                    state: HashMap::new(),
                    next: targets,
                },
            );
        } else {
            real_name = name;
            network.insert(name, Module::Broadcast { next: targets });
        }
        for target in outputs.split(", ") {
            reverse.entry(target).or_insert(Vec::new()).push(real_name);
        }
    }
    for (name, module) in &mut network {
        if let Module::Conjunction { state, .. } = module {
            state.extend(reverse[name].iter().map(|&x| (x, false)));
        }
    }
    let mut count_low = 0i64;
    let mut count_high = 0i64;
    for _ in 0..1000 {
        let mut queue = VecDeque::new();
        queue.push_back(("button", "broadcaster", false));
        while let Some((from, to, high)) = queue.pop_front() {
            if high {
                count_high += 1;
            } else {
                count_low += 1;
            }
            if let Some(target) = network.get_mut(to) {
                match target {
                    Module::Broadcast { next } => {
                        for target in next {
                            queue.push_back((to, target, high));
                        }
                    }
                    Module::FlipFlop { state, next } => {
                        if !high {
                            *state = !*state;
                            for target in next {
                                queue.push_back((to, target, *state));
                            }
                        }
                    }
                    Module::Conjunction { state, next } => {
                        *state.get_mut(from).unwrap() = high;
                        let signal = !state.values().all(|x| *x);
                        for target in next {
                            queue.push_back((to, target, signal));
                        }
                    }
                }
            }
        }
    }
    println!("Result: {}", count_low * count_high);
}
