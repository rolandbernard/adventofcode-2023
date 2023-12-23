fn longest_path(map: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, r: usize, c: usize) -> i64 {
    let mut result = i64::MIN;
    if r == map.len() - 1 {
        result = 0;
    } else if r < map.len() && c < map[r].len() && map[r][c] != '#' && !visited[r][c] {
        visited[r][c] = true;
        match map[r][c] {
            '>' => result = longest_path(map, visited, r, c + 1) + 1,
            'v' => result = longest_path(map, visited, r + 1, c) + 1,
            '^' => result = longest_path(map, visited, r - 1, c) + 1,
            '<' => result = longest_path(map, visited, r, c - 1) + 1,
            '.' => {
                let tmp = vec![
                    longest_path(map, visited, r, c + 1),
                    longest_path(map, visited, r, c.wrapping_sub(1)),
                    longest_path(map, visited, r + 1, c),
                    longest_path(map, visited, r.wrapping_sub(1), c),
                ];
                result = tmp.into_iter().max().unwrap() + 1;
            }
            _ => unreachable!(),
        }
        visited[r][c] = false;
    }
    return result;
}

fn main() {
    let mut map = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        map.push(line.chars().collect::<Vec<_>>());
    }
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    let start = map[0].iter().position(|&c| c != '#').unwrap();
    println!("Result: {}", longest_path(&map, &mut visited, 0, start));
}
