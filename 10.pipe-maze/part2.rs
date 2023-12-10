use std::collections::VecDeque;

fn get_connections(cell: char) -> &'static [(i64, i64)] {
    match cell {
        'S' => &[(1, 0), (0, 1), (-1, 0), (0, -1)],
        '|' => &[(1, 0), (-1, 0)],
        '-' => &[(0, 1), (0, -1)],
        'L' => &[(-1, 0), (0, 1)],
        'J' => &[(-1, 0), (0, -1)],
        '7' => &[(1, 0), (0, -1)],
        'F' => &[(1, 0), (0, 1)],
        '.' => &[],
        _ => unreachable!("unknown cell type"),
    }
}

fn main() {
    let mut map = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        map.push(line.chars().collect::<Vec<_>>());
    }
    let start;
    'start_search: {
        for (i, line) in map.iter().enumerate() {
            for (j, &cell) in line.iter().enumerate() {
                if cell == 'S' {
                    start = (i as i64, j as i64);
                    break 'start_search;
                }
            }
        }
        unreachable!("no start location found");
    }
    for rep in ['|', '-', 'L', 'J', '7', 'F'] {
        if get_connections(rep).iter().all(|&(di, dj)| {
            let (ni, nj) = (start.0 + di, start.1 + dj);
            ni >= 0
                && ni < map.len() as i64
                && nj >= 0
                && nj < map[ni as usize].len() as i64
                && get_connections(map[ni as usize][nj as usize]).contains(&(-di, -dj))
        }) {
            map[start.0 as usize][start.1 as usize] = rep;
        }
    }
    let mut pipe = vec![vec![false; map[0].len()]; map.len()];
    pipe[start.0 as usize][start.1 as usize] = true;
    let mut queue = VecDeque::new();
    queue.push_back(start);
    while let Some((i, j)) = queue.pop_front() {
        for (di, dj) in get_connections(map[i as usize][j as usize]) {
            let (ni, nj) = (i + di, j + dj);
            if ni >= 0
                && ni < map.len() as i64
                && nj >= 0
                && nj < map[ni as usize].len() as i64
                && get_connections(map[ni as usize][nj as usize]).contains(&(-di, -dj))
            {
                let nd = &mut pipe[ni as usize][nj as usize];
                if !*nd {
                    *nd = true;
                    queue.push_back((ni, nj));
                }
            }
        }
    }
    let mut count = 0;
    for i in 0..map.len() {
        let mut inside = false;
        for j in 0..map[i].len() {
            if !pipe[i][j] {
                if inside {
                    count += 1;
                }
            } else if ['|', 'F', '7'].contains(&map[i][j]) {
                inside = !inside;
            }
        }
    }
    println!("Result: {}", count);
}
