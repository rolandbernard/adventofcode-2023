use std::{collections::HashMap, io::Read, ops::Range};

fn count_options<'a>(
    workflows: &'a HashMap<&'a str, Vec<&str>>,
    workflow: &'a str,
    mut state: HashMap<&'a str, Range<i64>>,
) -> usize {
    if workflow == "R" {
        return 0;
    } else if workflow == "A" {
        return state.into_values().map(|r| r.count()).product();
    } else {
        let mut count = 0;
        for &instr in &workflows[workflow] {
            if let Some((cond, target)) = instr.split_once(':') {
                if let Some((prop, value)) = cond.split_once('<') {
                    let value = value.parse::<i64>().unwrap();
                    let mut copy = state.clone();
                    copy.entry(prop).and_modify(|r| {
                        *r = r.start..r.end.min(value);
                    });
                    state.entry(prop).and_modify(|r| {
                        *r = r.start.max(value)..r.end;
                    });
                    count += count_options(workflows, target, copy);
                } else if let Some((prop, value)) = cond.split_once('>') {
                    let value = value.parse::<i64>().unwrap();
                    let mut copy = state.clone();
                    copy.entry(prop).and_modify(|r| {
                        *r = r.start.max(value + 1)..r.end;
                    });
                    state.entry(prop).and_modify(|r| {
                        *r = r.start..r.end.min(value + 1);
                    });
                    count += count_options(workflows, target, copy);
                } else {
                    panic!();
                }
            } else {
                count += count_options(workflows, instr, state);
                break;
            }
        }
        return count;
    }
}

fn execute(workflows: &HashMap<&str, Vec<&str>>) -> usize {
    return count_options(
        workflows,
        "in",
        HashMap::from([
            ("x", 1..4001),
            ("m", 1..4001),
            ("a", 1..4001),
            ("s", 1..4001),
        ]),
    );
}

fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();
    let mut workflows = HashMap::new();
    let (code, _data) = input.trim().split_once("\n\n").unwrap();
    for wf in code.lines() {
        let (name, code) = wf.split_once('{').unwrap();
        workflows.insert(name, code[..code.len() - 1].split(',').collect::<Vec<_>>());
    }
    println!("Result: {}", execute(&workflows));
}
