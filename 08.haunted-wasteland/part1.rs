use std::{collections::HashMap, io::Read};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let (instr, map_lines) = input.split_once("\n\n").unwrap();
    let mut map = HashMap::new();
    for line in map_lines.lines() {
        let (node, conn) = line.split_once(" = ").unwrap();
        let (left, right) = conn
            .trim_matches(|c| c == '(' || c == ')')
            .split_once(", ")
            .unwrap();
        map.insert(node, (left, right));
    }
    let mut pos = "AAA";
    let mut step = 0;
    for instr in instr.chars().cycle() {
        if instr == 'L' {
            pos = map[pos].0;
        } else {
            pos = map[pos].1;
        }
        step += 1;
        if pos == "ZZZ" {
            break;
        }
    }
    println!("Result: {}", step);
}
