fn main() {
    let lines = std::io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>();
    let time = lines[0]
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<f64>()
        .unwrap();
    let distance = lines[1]
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<f64>()
        .unwrap();
    let det = f64::sqrt(time * time - 4.0 * (distance + 1.0));
    let x1 = (time - det) / 2.0;
    let x2 = (time + det) / 2.0;
    let cur = 1 + (x2.floor().max(0.0) - x1.ceil().max(0.0)).max(0.0) as i64;
    println!("Result: {}", cur);
}
