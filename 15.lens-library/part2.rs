use std::{collections::HashMap, io::Read};

fn hash(data: &[u8]) -> u8 {
    let mut state = 0u8;
    for &b in data {
        state = state.wrapping_add(b).wrapping_mul(17);
    }
    return state;
}

fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();
    let mut values = HashMap::new();
    for (i, instr) in input.trim().split(',').enumerate() {
        if instr.contains('=') {
            let (label, power_str) = instr.split_once('=').unwrap();
            let power = power_str.parse::<u8>().unwrap();
            values
                .entry(label.to_owned())
                .and_modify(|(_, p)| *p = power)
                .or_insert((i, power));
        } else {
            values.remove(&instr[..instr.len() - 1]);
        }
    }
    let mut boxes = vec![Vec::new(); 256];
    let mut flat_values = values.into_iter().collect::<Vec<_>>();
    flat_values.sort_by_key(|(_, (i, _))| *i);
    for (l, (_, p)) in flat_values {
        boxes[hash(l.as_bytes()) as usize].push(p);
    }
    let mut sum = 0;
    for (bi, b) in boxes.into_iter().enumerate() {
        for (si, p) in b.into_iter().enumerate() {
            sum += (1 + bi) * (1 + si) * (p as usize);
        }
    }
    println!("Result: {}", sum);
}
