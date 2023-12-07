use std::convert::TryInto;

const CARDS: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

fn get_type(cards: &[i32; 5]) -> i32 {
    let mut counts = [0; 13];
    for &c in cards {
        counts[c as usize] += 1;
    }
    counts.sort_by_key(|k| -k);
    2 * counts[0] + (if counts[1] == 2 { 1 } else { 0 })
}

fn main() {
    let mut sets: Vec<([i32; 5], i64)> = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let (cards_string, bid_string) = line.split_once(" ").unwrap();
        let bid = bid_string.trim().parse::<i64>().unwrap();
        let cards = cards_string
            .trim()
            .chars()
            .map(|c| CARDS.iter().position(|&x| x == c).unwrap() as i32)
            .collect::<Vec<_>>();
        sets.push((cards.try_into().unwrap(), bid));
    }
    sets.sort_by_key(|(cards, _)| (get_type(cards), *cards));
    let mut cur = 0;
    for (i, (_, bid)) in sets.into_iter().enumerate() {
        cur += (i + 1) as i64 * bid;
    }
    println!("Result: {}", cur);
}
