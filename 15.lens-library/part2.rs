mod common;

fn main() {
    let mut boxes = vec![Vec::new(); 256];
    for instr in common::read_input() {
        if instr.contains('=') {
            let (label, power_str) = instr.split_once('=').unwrap();
            let power = power_str.parse::<u8>().unwrap();
            let hash = common::hash(label) as usize;
            if let Some((_, p)) = boxes[hash].iter_mut().find(|(l, _)| l == label) {
                *p = power;
            } else {
                boxes[hash].push((label.to_owned(), power));
            }
        } else {
            let label = &instr[..instr.len() - 1];
            let hash = common::hash(label) as usize;
            if let Some(idx) = boxes[hash].iter().position(|(l, _)| l == label) {
                boxes[hash].remove(idx);
            }
        }
    }
    let mut sum = 0;
    for (bi, b) in boxes.into_iter().enumerate() {
        for (si, (_, p)) in b.into_iter().enumerate() {
            sum += (1 + bi) * (1 + si) * (p as usize);
        }
    }
    println!("Result: {}", sum);
}
