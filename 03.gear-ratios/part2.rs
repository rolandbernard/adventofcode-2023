mod common;

fn main() {
    let (map, numbers, number_map) = common::parse_input();
    let mut cur = 0;
    for (i, line) in map.iter().enumerate() {
        for (j, &char) in line.iter().enumerate() {
            if char == '*' {
                let labels = common::labels_for(&number_map, i, j);
                if labels.len() == 2 {
                    cur += labels.into_iter().map(|idx| numbers[idx]).product::<i64>();
                }
            }
        }
    }
    println!("Result: {}", cur);
}
