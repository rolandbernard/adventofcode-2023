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

pub fn matches_per_card() -> Vec<usize> {
    let mut result = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let (winning, mine) = parse_line(&line);
        let matches = mine.iter().filter(|card| winning.contains(card)).count();
        result.push(matches);
    }
    return result;
}
