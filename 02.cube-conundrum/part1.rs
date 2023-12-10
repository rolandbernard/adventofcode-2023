mod common;

const MAX: [i32; 3] = [12, 13, 14];

fn main() {
    let mut cur = 0;
    for (game_id, max) in common::per_game_max() {
        if MAX.iter().zip(max.iter()).all(|(x, y)| x >= y) {
            cur += game_id;
        }
    }
    println!("Result: {}", cur);
}
