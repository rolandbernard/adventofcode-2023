fn main() {
    let mut map = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        map.push(line.chars().collect::<Vec<_>>());
    }
    loop {
        let mut change = false;
        for i in 1..map.len() {
            for j in 0..map[i].len() {
                if map[i][j] == 'O' && map[i - 1][j] == '.' {
                    map[i][j] = '.';
                    map[i - 1][j] = 'O';
                    change = true;
                }
            }
        }
        if !change {
            break;
        }
    }
    let mut load = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 'O' {
                load += map.len() - i;
            }
        }
    }
    println!("Result: {}", load);
}
