pub fn read_input() -> Vec<Vec<char>> {
    let mut map = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        map.push(line.chars().collect::<Vec<_>>());
    }
    return map;
}

fn trace(
    map: &Vec<Vec<char>>,
    reached: &mut Vec<Vec<[bool; 4]>>,
    mut i: isize,
    mut j: isize,
    mut dir: usize,
) -> usize {
    let mut visited = 0;
    loop {
        if reached[i as usize][j as usize] == [false; 4] {
            visited += 1;
        } else if reached[i as usize][j as usize][dir] {
            break;
        }
        reached[i as usize][j as usize][dir] = true;
        match (map[i as usize][j as usize], dir) {
            ('\\', _) => {
                dir ^= 2;
            }
            ('/', _) => {
                dir ^= 3;
            }
            ('|', 0 | 1) => {
                visited += trace(map, reached, i, j, 2);
                dir = 3;
            }
            ('-', 2 | 3) => {
                visited += trace(map, reached, i, j, 0);
                dir = 1;
            }
            _ => {}
        };
        i += [0, 0, 1, -1][dir];
        j += [1, -1, 0, 0][dir];
        if !(i >= 0 && i < map.len() as isize && j >= 0 && j < map[i as usize].len() as isize) {
            break;
        }
    }
    return visited;
}

pub fn count_energy(map: &Vec<Vec<char>>, entry: (isize, isize, usize)) -> usize {
    let mut reached = vec![vec![[false; 4]; map[0].len()]; map.len()];
    return trace(map, &mut reached, entry.0, entry.1, entry.2);
}
