use std::io::Read;

pub fn hash(data: &str) -> u8 {
    let mut state = 0u8;
    for b in data.bytes() {
        state = state.wrapping_add(b).wrapping_mul(17);
    }
    return state;
}

pub fn read_input() -> Vec<String> {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().split(',').map(|e| e.to_owned()).collect();
}
