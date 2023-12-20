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

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64, i64, i64) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);
    while r != 0 {
        let q = old_r / r;
        (old_r, r) = (r, old_r - q * r);
        (old_s, s) = (s, old_s - q * s);
        (old_t, t) = (t, old_t - q * t);
    }
    (old_r, old_s, old_t, s, t)
}

fn find_offset(o: i64, p: i64, s: i64, r: i64) -> Option<i64> {
    let (gcd, x, y, zx, zy) = extended_gcd(p, r);
    if (s - o) % gcd != 0 {
        return None;
    }
    let off;
    if o > s {
        off = o + ((s - o) / gcd * x).rem_euclid(zx) * p;
    } else {
        off = s + ((o - s) / gcd * y).rem_euclid(zy) * r;
    }
    return Some(off);
}

fn lcm(a: i64, b: i64) -> i64 {
    a / extended_gcd(a, b).0 * b
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
    let mut last_special = HashMap::new();
    for press in 1..10_000 {
        let mut queue = VecDeque::new();
        queue.push_back(("button", "broadcaster", false));
        while let Some((from, to, high)) = queue.pop_front() {
            for special in ["kd", "zf", "vg", "gs"] {
                if !high && to == special {
                    if let Some((last, _)) = last_special.get(special) {
                        last_special.insert(special, (press, press - last));
                    } else {
                        last_special.insert(special, (press, 0));
                    }
                }
            }
            if !high && to == "rx" {
                println!("Result: {press}");
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
    let mut step = 0;
    let mut rep = 1;
    for (offset, period) in last_special.into_values() {
        (step, rep) = (
            find_offset(step, rep, offset, period).unwrap(),
            lcm(rep, period),
        );
    }
    println!("Result: {step}");
}
