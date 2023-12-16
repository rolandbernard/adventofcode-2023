fn main() {
    let mut map = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        map.push(line.chars().collect::<Vec<_>>());
    }
    let mut reachable = vec![vec![[false; 4]; map[0].len()]; map.len()];
    reachable[0][0][0] = true;
    let mut queue = Vec::new();
    queue.push((0, 0, 0));
    while let Some((i, j, dir)) = queue.pop() {
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
                    reachable[ni as usize][nj as usize][dir] = true;
                    queue.push((ni, nj, dir));
                }
            }
        }
    }
    let count = reachable
        .into_iter()
        .flatten()
        .filter(|x| x.contains(&true))
        .count();
    println!("Result: {}", count);
}
