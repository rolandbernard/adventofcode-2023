use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    let mut map = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        map.push(
            line.chars()
                .map(|e| e.to_digit(10).unwrap())
                .collect::<Vec<_>>(),
        );
    }
    let mut dist = vec![vec![[[u32::MAX; 11]; 4]; map[0].len()]; map.len()];
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, (0, 0, 0, 0))));
    while let Some(Reverse((d, (r, c, dir, dc)))) = queue.pop() {
        if r == map.len() - 1 && c == map[r].len() - 1 && dc >= 4 {
            println!("Result: {d}",);
            break;
        }
        for new_dir in [(dir + 3) % 4, (dir + 1) % 4, dir] {
            if (new_dir == dir && dc < 10) || (new_dir != dir && dc >= 4) {
                let nr = r as isize + [0, 1, 0, -1][new_dir];
                let nc = c as isize + [1, 0, -1, 0][new_dir];
                if nr >= 0 && nr < map.len() as isize && nc >= 0 && nc < map[r].len() as isize {
                    let (nr, nc) = (nr as usize, nc as usize);
                    let ndc = if new_dir == dir { dc + 1 } else { 1 };
                    let nd = d + map[nr][nc];
                    if nd < dist[nr][nc][new_dir][ndc] {
                        dist[nr][nc][new_dir][ndc] = nd;
                        queue.push(Reverse((nd, (nr, nc, new_dir, ndc))));
                    }
                }
            }
        }
    }
}
