use std::{cmp::Reverse, collections::BinaryHeap};

pub fn read_input() -> Vec<Vec<u32>> {
    let mut map = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        map.push(
            line.chars()
                .map(|e| e.to_digit(10).unwrap())
                .collect::<Vec<_>>(),
        );
    }
    return map;
}

pub fn optimal_loss(map: &Vec<Vec<u32>>, min_step: isize, max_step: isize) -> u32 {
    let mut dist = vec![vec![[u32::MAX; 4]; map[0].len()]; map.len()];
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, (0, 0, 0))));
    queue.push(Reverse((0, (0, 0, 1))));
    while let Some(Reverse((d, (r, c, dir)))) = queue.pop() {
        if r == map.len() - 1 && c == map[r].len() - 1 {
            return d;
        }
        for new_dir in [(dir + 3) % 4, (dir + 1) % 4] {
            let (dr, dc) = [(0, 1), (1, 0), (0, -1), (-1, 0)][new_dir];
            let mut nd = d;
            let mut nr = r as isize;
            let mut nc = c as isize;
            for step in 1..=max_step {
                nr += dr;
                nc += dc;
                if nr >= 0 && nr < map.len() as isize && nc >= 0 && nc < map[r].len() as isize {
                    let (nr, nc) = (nr as usize, nc as usize);
                    nd += map[nr][nc];
                    if step >= min_step && nd < dist[nr][nc][new_dir] {
                        dist[nr][nc][new_dir] = nd;
                        queue.push(Reverse((nd, (nr, nc, new_dir))));
                    }
                } else {
                    break;
                }
            }
        }
    }
    unreachable!();
}
