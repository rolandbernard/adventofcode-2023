fn parse_line(line: &str) -> (Vec<i32>, Vec<i32>) {
    let (_, content) = line.split_once(": ").unwrap();
    let (winning, having) = content.split_once(" | ").unwrap();
    let winning_numbers = winning
        .split(" ")
        .filter(|e| !e.trim().is_empty())
        .map(|e| e.trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let numbers = having
        .split(" ")
        .filter(|e| !e.trim().is_empty())
        .map(|e| e.trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    (winning_numbers, numbers)
}

fn main() {
    let lines = std::io::stdin().lines().collect::<Vec<_>>();
    let mut counts = vec![1; lines.len()];
    for (i, line) in lines.into_iter().enumerate() {
        let line = line.unwrap();
        let (winning, mine) = parse_line(&line);
        let matches = mine.iter().filter(|card| winning.contains(card)).count();
        for j in 1..=matches {
            counts[i + j] += counts[i];
        }
    }
    println!("Result: {}", counts.iter().sum::<i64>());
}
