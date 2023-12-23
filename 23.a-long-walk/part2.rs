mod common;

fn main() {
    let (start, end, network) = common::extract_network(&common::read_input(), false);
    println!("Result: {}", common::longest_path(&network, start, end));
}
