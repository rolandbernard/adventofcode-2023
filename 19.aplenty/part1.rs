use std::{collections::HashMap, io::Read};

fn run_workflow<'a>(wf: &Vec<&'a str>, elem: &HashMap<&str, i64>) -> &'a str {
    for instr in wf {
        if let Some((cond, target)) = instr.split_once(':') {
            if let Some((prop, value)) = cond.split_once('<') {
                if elem[prop] < value.parse().unwrap() {
                    return target;
                }
            } else if let Some((prop, value)) = cond.split_once('>') {
                if elem[prop] > value.parse().unwrap() {
                    return target;
                }
            } else {
                panic!();
            }
        } else {
            return instr;
        }
    }
    panic!();
}

fn execute(workflows: &HashMap<&str, Vec<&str>>, elem: &HashMap<&str, i64>) -> bool {
    let mut wf = &workflows["in"];
    loop {
        let target = run_workflow(wf, elem);
        if target == "A" {
            return true;
        } else if target == "R" {
            return false;
        } else {
            wf = &workflows[target];
        }
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();
    let mut workflows = HashMap::new();
    let (code, data) = input.trim().split_once("\n\n").unwrap();
    for wf in code.lines() {
        let (name, code) = wf.split_once('{').unwrap();
        workflows.insert(name, code[..code.len() - 1].split(',').collect::<Vec<_>>());
    }
    let mut sum = 0;
    for data in data.lines() {
        let mut part = HashMap::new();
        for prop in data[1..data.len() - 1].split(',') {
            let (name, value) = prop.split_once('=').unwrap();
            part.insert(name, value.parse::<i64>().unwrap());
        }
        if execute(&workflows, &part) {
            sum += part.into_values().sum::<i64>();
        }
    }
    println!("Result: {}", sum);
}
