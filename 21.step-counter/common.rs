pub fn read_input() -> (Vec<Vec<char>>, (i64, i64)) {
    let mut map = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        map.push(line.chars().collect::<Vec<_>>());
    }
    for (i, line) in map.iter_mut().enumerate() {
        for (j, cell) in line.iter_mut().enumerate() {
            if *cell == 'S' {
                *cell = '.';
                return (map, (i as i64, j as i64));
            }
        }
    }
    unreachable!();
}
