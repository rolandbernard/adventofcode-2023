mod common;

fn main() {
    let (seed_numbers, mappings) = common::parse_input();
    let whats_needed = common::Ranges::from_iter(seed_numbers.chunks(2).map(|x| (x[0], x[1])));
    println!(
        "Result: {}",
        common::min_location_for(whats_needed, mappings)
    );
}
