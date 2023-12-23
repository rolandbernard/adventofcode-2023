use std::collections::VecDeque;

mod common;

fn main() {
    let (map, start) = common::read_input();
    let mut reachable = vec![vec![[false; 2]; map[0].len()]; map.len()];
    reachable[start.0 as usize][start.1 as usize][0] = true;
    let mut queue = VecDeque::new();
    queue.push_back((0, start.0, start.1));
    while let Some((d, i, j)) = queue.pop_front() {
        if d > 64 {
            break;
        }
        for (di, dj) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let (ni, nj) = (i + di, j + dj);
            if ni >= 0
                && ni < map.len() as i64
                && nj >= 0
                && nj < map[ni as usize].len() as i64
                && map[ni as usize][nj as usize] == '.'
                && !reachable[ni as usize][nj as usize][(d + 1) % 2]
            {
                reachable[ni as usize][nj as usize][(d + 1) % 2] = true;
                queue.push_back((d + 1, ni, nj));
            }
        }
    }
    let mut count = 0;
    for row in reachable {
        for cell in row {
            if cell[0] {
                count += 1;
            }
        }
    }
    println!("Result: {count}");
}
