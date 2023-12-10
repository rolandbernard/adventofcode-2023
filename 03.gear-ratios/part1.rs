use std::collections::HashSet;

mod common;

fn main() {
    let (map, numbers, number_map) = common::parse_input();
    let mut unique = HashSet::new();
    for (i, line) in map.iter().enumerate() {
        for (j, &char) in line.iter().enumerate() {
            if !char.is_digit(10) && char != '.' {
                unique.extend(common::labels_for(&number_map, i, j).into_iter());
            }
        }
    }
    let cur = unique.into_iter().map(|idx| numbers[idx]).sum::<i64>();
    println!("Result: {}", cur);
}
