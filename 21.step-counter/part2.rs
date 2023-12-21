use std::collections::VecDeque;

fn count_options<F: Fn(i64, i64) -> bool>(reachable: &Vec<Vec<bool>>, cond: F) -> i64 {
    let mut count = 0;
    for (i, row) in reachable.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cond(i as i64, j as i64) && cell {
                count += 1;
            }
        }
    }
    return count;
}

fn main() {
    let mut map = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        map.push(line.chars().collect::<Vec<_>>());
    }
    let mut reachable = vec![vec![vec![false; map[0].len()]; map.len()]; 2];
    reachable[0][65][65] = true;
    let mut queue = VecDeque::new();
    queue.push_back((0, 65, 65));
    while let Some((d, i, j)) = queue.pop_front() {
        for (di, dj) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let (ni, nj) = (i + di, j + dj);
            if ni >= 0
                && ni < map.len() as i64
                && nj >= 0
                && nj < map[ni as usize].len() as i64
                && map[ni as usize][nj as usize] == '.'
                && !reachable[(d + 1) % 2][ni as usize][nj as usize]
            {
                reachable[(d + 1) % 2][ni as usize][nj as usize] = true;
                queue.push_back((d + 1, ni, nj));
            }
        }
    }
    let steps = 26501365;
    let full_odd = count_options(&reachable[1], |_, _| true);
    let full_even = count_options(&reachable[0], |_, _| true);
    let large_tl_odd = count_options(&reachable[1], |i, j| i + j <= 196);
    let large_bl_odd = count_options(&reachable[1], |i, j| (130 - i) + j <= 196);
    let large_tr_odd = count_options(&reachable[1], |i, j| i + (130 - j) <= 196);
    let large_br_odd = count_options(&reachable[1], |i, j| (130 - i) + (130 - j) <= 196);
    let small_tl_even = count_options(&reachable[0], |i, j| i + j <= 65);
    let small_bl_even = count_options(&reachable[0], |i, j| (130 - i) + j <= 65);
    let small_tr_even = count_options(&reachable[0], |i, j| i + (130 - j) <= 65);
    let small_br_even = count_options(&reachable[0], |i, j| (130 - i) + (130 - j) <= 65);
    let large_t = count_options(&reachable[1], |i, j| (130 - i) + (65 - j).abs() <= 130);
    let large_b = count_options(&reachable[1], |i, j| i + (65 - j).abs() <= 130);
    let large_l = count_options(&reachable[1], |i, j| (65 - i).abs() + (130 - j) <= 130);
    let large_r = count_options(&reachable[1], |i, j| (65 - i).abs() + j <= 130);
    let mul = steps / 131i64;
    let count = (mul / 2 * 2).pow(2) * full_even
        + ((mul + 1) / 2 * 2 - 1).pow(2) * full_odd
        + large_t
        + large_b
        + large_l
        + large_r
        + mul * (small_tl_even + small_bl_even + small_tr_even + small_br_even)
        + (mul - 1) * (large_tl_odd + large_bl_odd + large_tr_odd + large_br_odd);
    println!("Result: {count}");
}
