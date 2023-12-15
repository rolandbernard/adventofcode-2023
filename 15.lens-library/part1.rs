mod common;

fn main() {
    let mut sum = 0;
    for instr in common::read_input() {
        sum += common::hash(&instr) as u64;
    }
    println!("Result: {}", sum);
}
