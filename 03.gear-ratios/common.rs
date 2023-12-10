use std::{collections::HashSet, iter};

pub fn parse_input() -> (Vec<Vec<char>>, Vec<i64>, Vec<Vec<usize>>) {
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
    return (map, numbers, number_map);
}

pub fn labels_for(map: &Vec<Vec<usize>>, i: usize, j: usize) -> HashSet<usize> {
    let mut labels = HashSet::new();
    for di in -1..=1 {
        for dj in -1..=1 {
            let ni = i as i64 + di;
            let nj = j as i64 + dj;
            if ni >= 0 && ni < map.len() as i64 && nj >= 0 && nj < map[0].len() as i64 {
                labels.insert(map[ni as usize][nj as usize]);
            }
        }
    }
    labels.remove(&0);
    return labels;
}
