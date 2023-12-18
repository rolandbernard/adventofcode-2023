mod common;

fn main() {
    let mut points = Vec::new();
    let (mut x, mut y) = (0, 0);
    points.push((x, y));
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let (dir, rest) = line.split_once(' ').unwrap();
        let (len, _color) = rest.split_once(' ').unwrap();
        let (dx, dy) = match dir {
            "R" => (0, 1),
            "D" => (1, 0),
            "L" => (0, -1),
            "U" => (-1, 0),
            _ => unreachable!(),
        };
        let len = len.parse::<i64>().unwrap();
        (x, y) = (x + len * dx, y + len * dy);
        points.push((x, y));
    }
    println!("Result: {}", common::area_from_points(points));
}
