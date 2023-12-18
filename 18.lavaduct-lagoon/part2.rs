mod common;

fn main() {
    let mut points = Vec::new();
    let (mut x, mut y) = (0, 0);
    points.push((x, y));
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let (_dir, rest) = line.split_once(' ').unwrap();
        let (_len, instr) = rest.split_once(' ').unwrap();
        let len = i64::from_str_radix(&instr[2..instr.len() - 2], 16).unwrap();
        let (dx, dy) = match &instr[instr.len() - 2..instr.len() - 1] {
            "0" => (0, 1),
            "1" => (1, 0),
            "2" => (0, -1),
            "3" => (-1, 0),
            _ => unreachable!(),
        };
        (x, y) = (x + len * dx, y + len * dy);
        points.push((x, y));
    }
    println!("Result: {}", common::area_from_points(points));
}
