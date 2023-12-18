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
    let mut count0 = 0;
    let mut count1 = 0;
    for (&(x0, y0), &(x1, y1)) in points.iter().zip(points.iter().skip(1)) {
        count0 += (x0 * y1) - (x1 * y0);
        count1 += (x1 - x0).abs() + (y1 - y0).abs();
    }
    println!("Result: {}", count0.abs() / 2 + count1 / 2 + 1);
}
