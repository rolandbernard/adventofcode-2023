mod common;

fn main() {
    let mut cur = 0;
    for matches in common::matches_per_card() {
        if matches > 0 {
            cur += 1 << (matches - 1);
        }
    }
    println!("Result: {}", cur);
}
