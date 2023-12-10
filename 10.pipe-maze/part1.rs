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
    let mut dist = vec![vec![i64::MAX; map[0].len()]; map.len()];
    dist[start.0 as usize][start.1 as usize] = 0;
    let mut queue = VecDeque::new();
    queue.push_back(start);
    while let Some((i, j)) = queue.pop_front() {
        let d = dist[i as usize][j as usize];
        for (di, dj) in get_connections(map[i as usize][j as usize]) {
            let (ni, nj) = (i + di, j + dj);
            if ni >= 0
                && ni < map.len() as i64
                && nj >= 0
                && nj < map[ni as usize].len() as i64
                && get_connections(map[ni as usize][nj as usize]).contains(&(-di, -dj))
            {
                let nd = &mut dist[ni as usize][nj as usize];
                if *nd > d + 1 {
                    *nd = d + 1;
                    queue.push_back((ni, nj));
                }
            }
        }
    }
    println!(
        "Result: {}",
        dist.iter()
            .flat_map(|line| line.iter())
            .copied()
            .filter(|&x| x != i64::MAX)
            .max()
            .unwrap()
    );
}
