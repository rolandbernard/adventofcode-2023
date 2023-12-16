mod common;

fn main() {
    println!(
        "Result: {}",
        common::count_energy(&common::read_input(), (0, 0, 0))
    );
}
