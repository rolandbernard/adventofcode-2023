use std::collections::HashMap;

fn cycle(map: &mut Vec<Vec<u8>>) {
    for (di, dj) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
        loop {
            let mut change = false;
            for i in 0..map.len() {
                for j in 0..map[i].len() {
                    let ni = i as isize + di;
                    let nj = j as isize + dj;
                    if ni >= 0
                        && ni < map.len() as isize
                        && nj >= 0
                        && nj < map[i].len() as isize
                        && map[i][j] == b'O'
                        && map[ni as usize][nj as usize] == b'.'
                    {
                        map[i][j] = b'.';
                        map[ni as usize][nj as usize] = b'O';
                        change = true;
                    }
                }
            }
            if !change {
                break;
            }
        }
    }
}

fn main() {
    let mut map = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        map.push(line.bytes().collect::<Vec<_>>());
    }
    let mut seen = HashMap::new();
    let n = 1_000_000_000;
    for step in 1..n {
        cycle(&mut map);
        if let Some(first) = seen.get(&map) {
            if (n - first) % (step - first) == 0 {
                break;
            }
        } else {
            seen.insert(map.clone(), step);
        }
    }
    let mut load = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == b'O' {
                load += map.len() - i;
            }
        }
    }
    println!("Result: {}", load);
}
