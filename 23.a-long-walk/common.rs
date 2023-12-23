use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet, VecDeque},
};

fn neighbors<'a>(
    map: &'a Vec<Vec<char>>,
    r: usize,
    c: usize,
    directed: bool,
) -> impl Iterator<Item = (usize, usize)> + 'a {
    (if directed && map[r][c] != '.' {
        match map[r][c] {
            '>' => vec![(r, c + 1)],
            'v' => vec![(r + 1, c)],
            '^' => vec![(r - 1, c)],
            '<' => vec![(r, c - 1)],
            _ => unreachable!(),
        }
    } else {
        vec![
            (r, c + 1),
            (r + 1, c),
            (r, c.wrapping_sub(1)),
            (r.wrapping_sub(1), c),
        ]
    })
    .into_iter()
    .filter(move |&(nr, nc)| nr < map.len() && nc < map[nr].len() && map[nr][nc] != '#')
}

fn distances_from(
    map: &Vec<Vec<char>>,
    start: (usize, usize),
    points: &HashMap<(usize, usize), usize>,
    directed: bool,
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
            for (nr, nc) in neighbors(map, r, c, directed) {
                if !visited.contains(&(nr, nc)) {
                    visited.insert((nr, nc));
                    queue.push_back((d + 1, (nr, nc)));
                }
            }
        }
    }
    result.sort_by_key(|(_, d)| Reverse(*d));
    return result;
}

pub fn read_input() -> Vec<Vec<char>> {
    let mut map = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        map.push(line.chars().collect::<Vec<_>>());
    }
    return map;
}

pub fn extract_network(
    map: &Vec<Vec<char>>,
    directed: bool,
) -> (usize, usize, Vec<Vec<(usize, i64)>>) {
    let mut points = Vec::new();
    for (r, row) in map.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if cell != '#' {
                if neighbors(&map, r, c, directed).count() > 2 || r == 0 || r == map.len() - 1 {
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
        network.push(distances_from(&map, p, &points_map, directed));
    }
    assert!(network.len() < 64);
    let start = map[0].iter().position(|&c| c != '#').unwrap();
    let end = map.last().unwrap().iter().position(|&c| c != '#').unwrap();
    return (
        points_map[&(0, start)],
        points_map[&(map.len() - 1, end)],
        network,
    );
}

fn reachability(network: &Vec<Vec<(usize, i64)>>, subgraph: u64, start: usize) -> i64 {
    let mut max_each = 0;
    let mut visited = 0;
    let mut queue = VecDeque::new();
    queue.push_back(start);
    while let Some(pos) = queue.pop_front() {
        let mut max = [0, 0];
        for &(next, dist) in &network[pos] {
            if subgraph & (1 << next) == 0 {
                if dist > max[0] {
                    max[1] = max[0];
                    max[0] = dist;
                } else if dist > max[1] {
                    max[1] = dist;
                }
                if visited & (1 << next) == 0 {
                    visited |= 1 << next;
                    queue.push_back(next);
                }
            }
        }
        max_each += max[0] + max[1];
    }
    return max_each / 2;
}

fn longest_path_helper(
    network: &Vec<Vec<(usize, i64)>>,
    visited: u64,
    start: usize,
    end: usize,
    current: i64,
    max: i64,
) -> i64 {
    if start == end {
        return current;
    } else if (visited & (1 << start)) == 0 {
        let reachable = reachability(network, visited, start);
        if current + reachable <= max {
            return max;
        } else {
            let mut result = max;
            for &(next, dist) in &network[start] {
                result = result.max(longest_path_helper(
                    network,
                    visited | (1 << start),
                    next,
                    end,
                    current + dist,
                    result,
                ));
            }
            return result;
        }
    } else {
        return i64::MIN;
    }
}

pub fn longest_path(network: &Vec<Vec<(usize, i64)>>, start: usize, end: usize) -> i64 {
    longest_path_helper(network, 0, start, end, 0, i64::MIN)
}
