use std::collections::{HashMap, HashSet, VecDeque};

fn neighbors<'a>(
    map: &'a Vec<Vec<char>>,
    r: usize,
    c: usize,
) -> impl Iterator<Item = (usize, usize)> + 'a {
    vec![
        (r, c + 1),
        (r + 1, c),
        (r, c.wrapping_sub(1)),
        (r.wrapping_sub(1), c),
    ]
    .into_iter()
    .filter(move |&(nr, nc)| nr < map.len() && nc < map[nr].len() && map[nr][nc] != '#')
}

fn distances_from(
    map: &Vec<Vec<char>>,
    start: (usize, usize),
    points: &HashMap<(usize, usize), usize>,
) -> Vec<(usize, i64)> {
    let mut visited = HashSet::new();
    visited.insert(start);
    let mut queue = VecDeque::new();
    queue.push_back((0, start));
    let mut result = Vec::new();
    while let Some((d, (r, c))) = queue.pop_front() {
        if start != (r, c) && points.contains_key(&(r, c)) {
            result.push((points[&(r, c)], d));
        } else {
            for (nr, nc) in neighbors(map, r, c) {
                if !visited.contains(&(nr, nc)) {
                    visited.insert((nr, nc));
                    queue.push_back((d + 1, (nr, nc)));
                }
            }
        }
    }
    return result;
}

fn longest_path(network: &Vec<Vec<(usize, i64)>>, visited: u64, start: usize, end: usize) -> i64 {
    if start == end {
        return 0;
    } else if visited & (1 << start) == 0 {
        let result = network[start]
            .iter()
            .map(|&(next, dist)| dist + longest_path(network, visited | (1 << start), next, end))
            .max()
            .unwrap_or(i64::MIN);
        return result;
    } else {
        return i64::MIN;
    }
}

fn main() {
    let mut map = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        map.push(line.chars().collect::<Vec<_>>());
    }
    let mut points = Vec::new();
    for (r, row) in map.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if cell != '#' {
                if neighbors(&map, r, c).count() > 2 || r == 0 || r == map.len() - 1 {
                    points.push((r, c));
                }
            }
        }
    }
    let points_map = points
        .iter()
        .enumerate()
        .map(|(i, p)| (*p, i))
        .collect::<HashMap<_, _>>();
    let mut network = Vec::new();
    for &p in &points {
        network.push(distances_from(&map, p, &points_map));
    }
    assert!(network.len() < 64);
    let start = map[0].iter().position(|&c| c != '#').unwrap();
    let end = map.last().unwrap().iter().position(|&c| c != '#').unwrap();
    println!(
        "Result: {}",
        longest_path(
            &network,
            0,
            points_map[&(0, start)],
            points_map[&(map.len() - 1, end)]
        )
    );
}
