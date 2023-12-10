mod common;

fn main() {
    let mut counts = Vec::new();
    for (i, matches) in common::matches_per_card().into_iter().enumerate() {
        counts.resize(counts.len().max(i + matches + 1), 1);
        for j in 1..=matches {
            counts[i + j] += counts[i];
        }
    }
    println!("Result: {}", counts.iter().sum::<i64>());
}
