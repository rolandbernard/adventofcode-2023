mod common;

fn main() {
    let map = common::read_input();
    println!("Result: {}", common::optimal_loss(&map, 4, 10));
}
