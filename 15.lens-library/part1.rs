use std::io::Read;

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
    let mut sum = 0;
    for instr in input.trim().split(',') {
        sum += hash(instr.as_bytes()) as u64;
    }
    println!("Result: {}", sum);
}
