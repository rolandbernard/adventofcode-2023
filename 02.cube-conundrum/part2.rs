mod common;

fn main() {
    let mut cur = 0;
    for (_, max) in common::per_game_max() {
        cur += max.iter().product::<i32>();
    }
    println!("Result: {}", cur);
}
