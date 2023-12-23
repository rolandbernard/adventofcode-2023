use std::io::Read;

mod common;

fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();
    let (mut network, _) = common::parse_input(&input);
    let mut count_low = 0i64;
    let mut count_high = 0i64;
    for _ in 0..1000 {
        common::evaluate_press(&mut network, |_, _, high| {
            if high {
                count_high += 1;
            } else {
                count_low += 1;
            }
            false
        });
    }
    println!("Result: {}", count_low * count_high);
}
