fn count_energy(map: &Vec<Vec<char>>, entry_at: (isize, isize, usize)) -> usize {
    let mut reachable = vec![vec![[false; 4]; map[0].len()]; map.len()];
    let mut queue = Vec::new();
    queue.push(entry_at);
    while let Some((i, j, dir)) = queue.pop() {
        reachable[i as usize][j as usize][dir] = true;
        let next = match (map[i as usize][j as usize], dir) {
            ('|', 0 | 1) => vec![2, 3],
            ('-', 2 | 3) => vec![0, 1],
            ('\\', 0) => vec![2],
            ('\\', 1) => vec![3],
            ('\\', 2) => vec![0],
            ('\\', 3) => vec![1],
            ('/', 0) => vec![3],
            ('/', 1) => vec![2],
            ('/', 2) => vec![1],
            ('/', 3) => vec![0],
            _ => vec![dir],
        };
        for dir in next {
            let (ndi, ndj) = [(0, 1), (0, -1), (1, 0), (-1, 0)][dir];
            let (ni, nj) = (i + ndi, j + ndj);
            if ni >= 0 && ni < map.len() as isize && nj >= 0 && nj < map[ni as usize].len() as isize
            {
                if !reachable[ni as usize][nj as usize][dir] {
                    queue.push((ni, nj, dir));
                }
            }
        }
    }
    return reachable
        .into_iter()
        .flatten()
        .filter(|x| x.contains(&true))
        .count();
}

fn main() {
    let mut map = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        map.push(line.chars().collect::<Vec<_>>());
    }
    let mut max = 0;
    for i in 0..map.len() as isize {
        max = max.max(count_energy(&map, (i, 0, 0)));
        max = max.max(count_energy(&map, (i, map[0].len() as isize - 1, 1)));
    }
    for j in 0..map[0].len() as isize {
        max = max.max(count_energy(&map, (0, j, 2)));
        max = max.max(count_energy(&map, (map.len() as isize - 1, j, 3)));
    }
    println!("Result: {max}");
}
