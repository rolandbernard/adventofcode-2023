use std::{collections::HashSet, iter};

fn main() {
    let map = std::io::stdin()
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut numbers = vec![0];
    let mut number_map = vec![vec![0; map[0].len()]; map.len()];
    for (i, line) in map.iter().enumerate() {
        let mut len = 0;
        let mut number = 0;
        for (j, &char) in line.iter().chain(iter::once(&'.')).enumerate() {
            if char.is_digit(10) {
                len += 1;
                number = 10 * number + char.to_digit(10).unwrap() as i64;
            } else {
                let idx = numbers.len();
                numbers.push(number);
                number_map[i][j - len..j].fill(idx);
                len = 0;
                number = 0;
            }
        }
    }
    let mut unique = HashSet::new();
    for (i, line) in map.iter().enumerate() {
        for (j, &char) in line.iter().enumerate() {
            if !char.is_digit(10) && char != '.' {
                for di in -1..=1 {
                    for dj in -1..=1 {
                        let ni = i as i64 + di;
                        let nj = j as i64 + dj;
                        if ni >= 0
                            && ni < number_map.len() as i64
                            && nj >= 0
                            && nj < number_map[0].len() as i64
                        {
                            unique.insert(number_map[ni as usize][nj as usize]);
                        }
                    }
                }
            }
        }
    }
    let cur = unique.into_iter().map(|idx| numbers[idx]).sum::<i64>();
    println!("Result: {}", cur);
}
